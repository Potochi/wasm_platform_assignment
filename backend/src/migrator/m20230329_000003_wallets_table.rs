use sea_orm_migration::prelude::*;

use super::m20230328_000001_users_table::User;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &'static str {
        "m20230329_000003_wallets_table"
    }
}

#[sea_orm_migration::async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Wallet::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Wallet::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Wallet::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-wallet-user_id")
                            .from(Wallet::Table, Wallet::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Wallet::Credits).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Wallet::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Wallet {
    Table,
    Id,
    UserId,
    Credits,
}
