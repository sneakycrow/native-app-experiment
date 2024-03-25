use sea_orm_migration::prelude::*;

use super::m20240325_202911_create_list_table::List;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ToDo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ToDo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ToDo::ListId).integer().not_null())
                    .col(ColumnDef::new(ToDo::Message).string().not_null())
                    .col(
                        ColumnDef::new(ToDo::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(ToDo::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-todo-list_id")
                            .from(ToDo::Table, ToDo::ListId)
                            .to(List::Table, List::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ToDo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ToDo {
    Table,
    Id,
    ListId,
    Message,
    CreatedAt,
    UpdatedAt,
}
