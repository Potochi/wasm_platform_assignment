use std::{net::SocketAddr, str::FromStr, sync::Arc};

use anyhow::anyhow;

use aws_common::api::errors::AwsError;
use axum::{
    routing::{delete, get, post},
    Extension, Router,
};

use sea_orm::{ConnectOptions, Database};

use aws_backend::utils::DbConn;
use aws_backend::{cache::ModuleCache, routes::modules::delete_module};
use tower_http::cors;

use aws_backend::routes::{
    functions::call_function,
    modules::{deploy_module, get_deployed_modules},
    user::{get_remaining_credits, login_user, register_user},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::try_init()
        .map_err(|_| anyhow!("Failed to install tracing_subscriber"))?;

    let mut db_opts = ConnectOptions::new(std::env::var("DB_URL").expect("DB_URL to be present"));
    db_opts.sqlx_logging(false);

    let db = Database::connect(db_opts).await?;
    let cache = ModuleCache::default();

    let db_conn = DbConn(Arc::new(db));

    let app = Router::new()
        .fallback(fallback)
        .nest(
            "/api/v1",
            Router::new()
                .nest(
                    "/auth",
                    Router::new()
                        .route("/register", post(register_user))
                        .route("/login", post(login_user)),
                )
                .nest(
                    "/user",
                    Router::new()
                        .route("/currency", get(get_remaining_credits))
                        .route("/modules", get(get_deployed_modules)),
                )
                .nest(
                    "/module",
                    Router::new().route("/deploy", post(deploy_module)).nest(
                        "/delete",
                        Router::new()
                            .route("/:id", delete(delete_module))
                            .layer(Extension(cache.clone())),
                    ),
                )
                .nest(
                    "/function",
                    Router::new()
                        .route("/call/:id/:func_name", post(call_function))
                        .layer(Extension(cache)),
                ),
        )
        .layer(Extension(db_conn))
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

async fn fallback(uri: axum::http::Uri) -> AwsError {
    AwsError::NotFound(Box::new(uri))
}
