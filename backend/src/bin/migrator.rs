use aws_backend::migrator;
use sea_orm::{ConnectOptions, Database};
use sea_orm_migration::MigratorTrait;
use tokio::time::{sleep, Duration};

async fn attempt_migrations() -> anyhow::Result<()> {
    let mut db_opts = ConnectOptions::new(std::env::var("DB_URL").expect("DB_URL to be present"));
    db_opts.sqlx_logging(true);

    let db = Database::connect(db_opts).await?;
    migrator::Migrator::up(&db, None).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::try_init()
        .map_err(|_| anyhow::anyhow!("Failed to install tracing_subscriber"))?;

    let sleep_secs = u64::from_str_radix(
        &std::env::var("RETRY_SLEEP_SECS").unwrap_or("5".to_owned()),
        10,
    )
    .map_err(|_| anyhow::anyhow!("Invalid sleep duration"))?;

    while let Err(_) = attempt_migrations().await {
        tracing::info!("Could not run migrations, retrying...");
        sleep(Duration::from_secs(sleep_secs)).await;
    }

    Ok(())
}
