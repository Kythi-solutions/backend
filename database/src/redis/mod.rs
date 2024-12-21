use std::env;

use actix_session::storage::RedisSessionStore;
use dotenvy::dotenv;

pub async fn create() -> Result<RedisSessionStore, anyhow::Error> {
    dotenv().ok();

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    RedisSessionStore::new(redis_url)
        .await
}