use aws_backend::{constants, migrator};
use sea_orm::{ConnectOptions, Database};
use sea_orm_migration::MigratorTrait;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut db_opts = ConnectOptions::new(
        std::env::var("DB_URL").unwrap_or(constants::DEFAULT_DB_URL.to_string()),
    );
    db_opts.sqlx_logging(false);

    let db = Database::connect(db_opts).await?;
    migrator::Migrator::up(&db, None).await?;

    Ok(())
}
