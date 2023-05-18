use aws_common::api::errors::AwsError;
use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    RequestPartsExt,
};
use jsonwebtoken::Validation;
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
    pub async fn to_jwt(&self) -> Result<String, AwsError> {
        tracing::info!("CACARE REQUEST");
        let client = reqwest::Client::new();

        let response = client
            .post("http://auth:3000/api/v1/sign")
            .json(self)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Resp {e:#?}");
                AwsError::JwtSignatureFailure
            })?;

        tracing::info!("Signature response = {response:#?}");

        let token = response.json::<String>().await.map_err(|e| {
            tracing::error!("Body {e:#?}");
            AwsError::JwtSignatureFailure
        })?;

        Ok(token)
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
            &Validation::new(jsonwebtoken::Algorithm::ES256),
        )
        .map_err(|e| {
            tracing::debug!("Token verification failed with {err}", err = e);

            AwsError::Unauthorized
        })?;

        Ok(token.claims)
    }
}

#[test]
fn test_jwt_keys() {
    let claims = AwsClaims {
        sub: "emi".to_string(),
        exp: (SystemTime::now().duration_since(UNIX_EPOCH).unwrap() + JWT_TOKEN_VALIDITY).as_secs()
            as usize,
        uid: 0i32,
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::ES256),
        &claims,
        &JWT_ENCODING_KEY,
    )
    .unwrap();

    println!("{token}");

    let _: AwsClaims = jsonwebtoken::decode(
        &token,
        &JWT_DECODING_KEY,
        &Validation::new(jsonwebtoken::Algorithm::ES256),
    )
    .unwrap()
    .claims;
}
