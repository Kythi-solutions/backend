use std::env;
use std::time::Duration;

use sea_orm::{DatabaseConnection, ConnectOptions, Database, DbErr};
use dotenvy::dotenv;


pub async fn create() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn_options = ConnectOptions::new(database_url.to_string());

    conn_options.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    Database::connect(conn_options).await
}