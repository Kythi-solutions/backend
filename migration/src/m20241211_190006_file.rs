use sea_orm_migration::{prelude::*, schema::*};
use crate::m20241211_185925_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(File::Table)
                    .if_not_exists()
                    .col(pk_auto(File::Id))
                    .col(integer(File::UserId))
                    .col(string(File::Name))
                    .col(string(File::Hash))
                    .col(string(File::Type))
                    .col(boolean(File::Starred))
                    .col(timestamp(File::CreatedAt))
                    .col(timestamp(File::LastModified))
                    .col(timestamp(File::LastOpened))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-file-user-id")
                            .from(File::Table, File::UserId)
                            .to(User::Table, User::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(File::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum File {
    Table,
    Id,
    UserId,
    Name,
    Hash,
    Type,
    Starred,
    CreatedAt,
    LastModified,
    LastOpened
}




