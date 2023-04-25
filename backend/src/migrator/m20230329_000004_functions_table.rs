use sea_orm_migration::prelude::*;

use super::m20230328_000002_modules_table::Module;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &'static str {
        "m20230329_000004_functions_table"
    }
}

#[sea_orm_migration::async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Function::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Function::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Function::ModuleId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-function-module_id")
                            .from(Function::Table, Function::ModuleId)
                            .to(Module::Table, Module::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Function::Name).string().not_null())
                    .col(ColumnDef::new(Function::Signature).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Function::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Function {
    Table,
    Id,
    Name,
    ModuleId,
    Signature,
}
