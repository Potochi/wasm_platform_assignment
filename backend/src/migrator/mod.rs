pub struct Migrator;

use sea_orm_migration::MigratorTrait;

pub mod m20230328_000001_users_table;
pub mod m20230328_000002_modules_table;
pub mod m20230329_000003_wallets_table;
pub mod m20230329_000004_functions_table;

#[sea_orm_migration::async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn sea_orm_migration::MigrationTrait>> {
        vec![
            Box::new(m20230328_000001_users_table::Migration),
            Box::new(m20230328_000002_modules_table::Migration),
            Box::new(m20230329_000003_wallets_table::Migration),
            Box::new(m20230329_000004_functions_table::Migration),
        ]
    }
}
