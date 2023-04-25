use sea_orm_migration::prelude::*;

use super::m20230328_000001_users_table::User;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &'static str {
        "m20230328_000002_modules_table"
    }
}

#[sea_orm_migration::async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Module::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Module::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Module::OwnerId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-endpoint-owner_id")
                            .from(Module::Table, Module::OwnerId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Module::CodeHash)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Module::WasmCode).binary().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Module::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Module {
    Table,
    Id,
    OwnerId,
    CodeHash,
    WasmCode,
}
