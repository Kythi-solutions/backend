use std::sync::Arc;
use sea_orm::{ColumnTrait, QueryFilter};

use super::Repository;
use crate::repository;
use crate::entity::user;

pub struct UserRepository {
    db: Arc<DatabaseConnection>
}

repository!(UserRepository, user::Entity, user::Model, user::ActiveModel);

impl UserRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn by_username(self, username: String) -> Result<Option<user::Model>, sea_orm::DbErr> {
        user::Entity::find().filter(user::Column::Username.eq(username)).one(self.db.as_ref()).await
    }
}