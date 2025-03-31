use sea_orm_migration::prelude::*;

#[derive(Iden)]
enum Vendors {
    Table,
    Id,
    Email,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Vendors::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Vendors::Id)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Vendors::Email)
                            .string()
                            .not_null()
                            .unique_key()
                            .char_len(200),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Vendors::Table).to_owned())
            .await
    }
}
