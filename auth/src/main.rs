use std::{net::SocketAddr, str::FromStr};

use anyhow::anyhow;
use axum::{extract::Json, http::StatusCode, routing::post, Router};
use jsonwebtoken::EncodingKey;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tower_http::cors;

lazy_static! {
    pub static ref JWT_ENCODING_KEY: EncodingKey = EncodingKey::from_ec_pem(
        std::fs::read_to_string(
            std::env::var("JWT_PRIVATE_KEY_PATH").expect("env var to be present")
        )
        .expect("to be able to read private key file")
        .as_bytes()
    )
    .expect("Private key to be valid");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AwsClaims {
    pub sub: String,
    pub exp: usize,
    pub uid: i32,
}

pub async fn fallback(_: axum::http::Uri) -> StatusCode {
    StatusCode::NOT_FOUND
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::try_init()
        .map_err(|_| anyhow!("Failed to install tracing_subscriber"))?;

    let app = Router::new()
        .fallback(fallback)
        .nest("/api/v1", Router::new().route("/sign", post(sign_claims)))
        .layer(cors::CorsLayer::very_permissive());

    let addr =
        SocketAddr::from_str(&std::env::var("LISTEN_ADDR").expect("LISTEN_ADDR to be present"))
            .map_err(|e| anyhow!(e))?;

    tracing::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn sign_claims(Json(body): Json<AwsClaims>) -> Result<Json<String>, StatusCode> {
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::ES256),
        &body,
        &JWT_ENCODING_KEY,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(token.into())
}
