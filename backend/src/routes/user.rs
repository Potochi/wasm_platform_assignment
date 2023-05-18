use std::time::{SystemTime, UNIX_EPOCH};

use aws_common::api::{errors::AwsError, responses::GetCreditsResponse};
use axum::{http::StatusCode, Extension};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter, TransactionTrait,
};
use serde::Deserialize;

use crate::{
    auth::jwt::{AwsClaims, JwtResponse},
    constants::{INITIAL_WALLET_CREDITS, JWT_TOKEN_VALIDITY, MINIMUM_PASSWORD_LENGTH},
    entities,
    extractors::WalletExtract,
    metrics::ACTIVE_USERS,
    utils::{password_secure_check, DbConn},
};

use argon2::{
    self,
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub async fn get_remaining_credits(
    WalletExtract(wallet): WalletExtract,
) -> Result<axum::Json<GetCreditsResponse>, AwsError> {
    Ok(axum::Json::from(GetCreditsResponse {
        credits: wallet.credits,
    }))
}

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

// TODO(Livian): Implement Refresh Tokens + Redis cache
pub async fn register_user(
    Extension(DbConn(db)): Extension<DbConn>,
    axum::extract::Json(Credentials { username, password }): axum::extract::Json<Credentials>,
) -> Result<StatusCode, AwsError> {
    tracing::debug!(
        "Registering user {user} with password {password}",
        user = username,
        password = password
    );

    if password.len() < MINIMUM_PASSWORD_LENGTH {
        return Err(AwsError::PasswordTooShort);
    }

    if !password_secure_check(&password) {
        return Err(AwsError::PasswordTooWeak);
    }

    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AwsError::UnknownServerError)?
        .to_string();

    db.transaction::<_, _, DbErr>(|txn| {
        Box::pin(async move {
            let new_user = entities::user::ActiveModel {
                username: ActiveValue::Set(username),
                password: ActiveValue::Set(hashed_password),
                ..Default::default()
            };

            let added_user = new_user.save(txn).await?;

            let new_wallet = entities::wallet::ActiveModel {
                user_id: added_user.id,
                credits: ActiveValue::set(INITIAL_WALLET_CREDITS),
                ..Default::default()
            };

            new_wallet.save(txn).await?;

            Ok(())
        })
    })
    .await
    .map_err(|_| AwsError::DuplicateUsername)?;

    ACTIVE_USERS.inc();

    Ok(StatusCode::OK)
}

pub async fn login_user(
    Extension(DbConn(db)): Extension<DbConn>,
    axum::extract::Json(Credentials { username, password }): axum::extract::Json<Credentials>,
) -> Result<axum::Json<JwtResponse>, AwsError> {
    let res = entities::user::Entity::find()
        .filter(entities::user::Column::Username.eq(&username))
        .one(&*db)
        .await
        .map_err(|_| AwsError::UnknownServerError)?
        .ok_or_else(|| AwsError::InvalidCredentials)?;

    let password_hash =
        PasswordHash::new(&res.password).map_err(|_| AwsError::UnknownServerError)?;

    Argon2::default()
        .verify_password(password.as_bytes(), &password_hash)
        .map_err(|_| AwsError::InvalidCredentials)?;

    tracing::debug!(
        "Login OK for user {user} with password {password}",
        user = username,
        password = password
    );

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AwsError::UnknownServerError)?
        + JWT_TOKEN_VALIDITY;

    let claims = AwsClaims {
        sub: username,
        exp: expiration.as_secs() as usize,
        uid: res.id,
    };

    Ok(axum::Json::from(JwtResponse {
        jwt: claims.to_jwt().await?,
    }))
}

pub async fn delete_account(
    claims: AwsClaims,
    Extension(DbConn(db)): Extension<DbConn>,
) -> Result<(), AwsError> {
    db.transaction::<_, _, DbErr>(|txn| {
        Box::pin(async move {
            let user = entities::user::ActiveModel {
                id: ActiveValue::Set(claims.uid),
                ..Default::default()
            };

            user.delete(txn).await?;

            Ok(())
        })
    })
    .await
    .map_err(|_| AwsError::DuplicateUsername)?;

    ACTIVE_USERS.dec();

    Ok(())
}
