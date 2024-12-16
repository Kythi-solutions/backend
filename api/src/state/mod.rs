use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct GlobalState {
    pub version: String,
    pub db: DatabaseConnection,
}