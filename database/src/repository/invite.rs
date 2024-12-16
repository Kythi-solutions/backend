use super::Repository;
use crate::repository_macro;

use entity::invite;

struct InviteRepository {
    db: DatabaseConnection
}

impl InviteRepository {
    fn new(db: DatabaseConnection) -> InviteRepository {
        InviteRepository { db }
    }
}

repository_macro!(invite, InviteRepository, i32);