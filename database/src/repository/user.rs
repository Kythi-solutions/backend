use crate::entity::prelude::User;
use super::Repository;
use crate::repository_macro;

use crate::entity::user;

use sea_orm::{ColumnTrait, QueryFilter};
use syn::token::Use;

pub struct UserRepository {
    db: DatabaseConnection
}

repository_macro!(user, UserRepository, i32);

impl UserRepository {
    pub fn new(db: DatabaseConnection) -> UserRepository {
        UserRepository { db }
    }

    pub async fn by_username(&self, username: String) ->  Result<Option<user::Model>, sea_orm::DbErr> {
        User::find().filter(user::Column::Username.eq(username)).one(&self.db).await
    }
}

impl Copy for UserRepository { }

impl Clone for UserRepository {
    fn clone(&self) -> UserRepository {
        *self
    }
}