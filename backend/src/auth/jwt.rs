use aws_common::api::errors::AwsError;
use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    RequestPartsExt,
};
use jsonwebtoken::{EncodingKey, Validation};
use serde::{Deserialize, Serialize};

use super::keys::JWT_DECODING_KEY;

#[derive(Serialize)]
pub struct JwtResponse {
    pub jwt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AwsClaims {
    pub sub: String,
    pub exp: usize,
    pub uid: i32,
}

impl AwsClaims {
    pub fn to_jwt(&self, encoding_key: &EncodingKey) -> Result<String, AwsError> {
        let bearer = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &self, encoding_key)
            .map_err(|_| AwsError::UnknownServerError)?;

        Ok(bearer)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AwsClaims
where
    S: Send + Sync,
{
    type Rejection = AwsError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let bearer = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AwsError::Unauthorized)?;

        tracing::debug!("Checking token {token}", token = bearer.token());

        let token = jsonwebtoken::decode::<AwsClaims>(
            bearer.token(),
            &JWT_DECODING_KEY,
            &Validation::default(),
        )
        .map_err(|e| {
            tracing::debug!("Token verification failed with {err}", err = e);

            AwsError::Unauthorized
        })?;

        Ok(token.claims)
    }
}
