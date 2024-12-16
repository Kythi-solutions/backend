use super::Repository;
use crate::repository_macro;

use entity::user;


struct UserRepository {
    db: DatabaseConnection
}

impl UserRepository {
    fn new(db: DatabaseConnection) -> UserRepository {
        UserRepository { db }
    }
}

repository_macro!(user, UserRepository, i32);