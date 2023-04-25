use aws_common::api::errors::AwsError;
use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::request::Parts,
    Extension,
};

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{auth::jwt::AwsClaims, entities, utils::DbConn};

#[derive(Deserialize)]
pub struct ModuleHashPathParam {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct FuncNamePathParam {
    pub func_name: String,
}

pub struct ModuleExtractor(pub entities::module::Model);

pub struct ModuleFunctionExtract {
    pub module: entities::module::Model,
    pub function: entities::function::Model,
}

pub struct WalletExtract(pub entities::wallet::Model);

#[async_trait]
impl<S> FromRequestParts<S> for WalletExtract
where
    S: Send + Sync,
{
    type Rejection = AwsError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        use axum::RequestPartsExt;
        use entities::wallet as Wal;

        let Extension(DbConn(db)) = parts
            .extract::<Extension<DbConn>>()
            .await
            .map_err(|_| AwsError::UnknownServerError)?;

        let user_claims = AwsClaims::from_request_parts(parts, state)
            .await
            .map_err(|_| AwsError::Unauthorized)?;

        Ok(Self(
            Wal::Entity::find()
                .filter(Wal::Column::UserId.eq(user_claims.uid))
                .one(&*db)
                .await
                .map_err(|_| AwsError::UnknownServerError)?
                .ok_or_else(|| AwsError::UnknownServerError)?,
        ))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ModuleFunctionExtract
where
    S: Send + Sync,
{
    type Rejection = AwsError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        use axum::RequestPartsExt;
        use entities::function as Func;

        let Extension(DbConn(db)) = parts
            .extract::<Extension<DbConn>>()
            .await
            .map_err(|_| AwsError::UnknownServerError)?;

        let ModuleExtractor(module) = ModuleExtractor::from_request_parts(parts, state).await?;

        let Path(FuncNamePathParam { func_name }) =
            Path::<FuncNamePathParam>::from_request_parts(parts, state)
                .await
                .map_err(|_| AwsError::NotFound(Box::new(parts.uri.clone())))?;

        Ok(Self {
            function: Func::Entity::find()
                .filter(Func::Column::ModuleId.eq(module.id))
                .filter(Func::Column::Name.eq(&func_name))
                .one(&*db)
                .await
                .map_err(|_| AwsError::UnknownServerError)?
                .ok_or_else(|| AwsError::FunctionNotFound(func_name))?,
            module,
        })
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ModuleExtractor
where
    S: Send + Sync,
{
    type Rejection = AwsError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        use axum::RequestPartsExt;
        use entities::module as Endp;

        let Extension(DbConn(db)) = parts
            .extract::<Extension<DbConn>>()
            .await
            .map_err(|_| AwsError::UnknownServerError)?;

        let Path(ModuleHashPathParam { id }) =
            Path::<ModuleHashPathParam>::from_request_parts(parts, state)
                .await
                .map_err(|_| AwsError::Unauthorized)?;

        let user_claims = AwsClaims::from_request_parts(parts, state)
            .await
            .map_err(|_| AwsError::Unauthorized)?;

        Ok(ModuleExtractor(
            Endp::Entity::find()
                .filter(Endp::Column::OwnerId.eq(user_claims.uid))
                .filter(Endp::Column::Id.eq(id))
                .one(&*db)
                .await
                .map_err(|_| AwsError::UnknownServerError)?
                .ok_or_else(|| AwsError::EndpointNotFound(id))?,
        ))
    }
}
