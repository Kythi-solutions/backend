use sea_orm_migration::{prelude::*, schema::*};

use super::m20241211_185925_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Invite::Table)
                    .if_not_exists()
                    .col(pk_auto(Invite::Id))
                    .col(integer(Invite::UserId))
                    .col(string(Invite::Code))
                    .col(boolean(Invite::Redeemed))
                    .col(timestamp(Invite::RedeemedBy))
                    .col(timestamp(Invite::RedeemedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-invite-user-id")
                            .from(Invite::Table, Invite::UserId)
                            .to(User::Table, User::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Invite::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Invite {
    Table,
    Id,
    UserId,
    Code,
    Redeemed,
    RedeemedBy,
    RedeemedAt
}
