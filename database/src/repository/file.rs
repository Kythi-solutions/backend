use super::Repository;
use crate::repository_macro;
use crate::entity::file;

struct FileRepository {
    db: DatabaseConnection
}

impl FileRepository {
    fn new(db: DatabaseConnection) -> FileRepository {
        FileRepository { db }
    }
}

repository_macro!(file, FileRepository, i32);