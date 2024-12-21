use std::env;

use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::cookie::Key;

pub fn create(store: RedisSessionStore) -> SessionMiddleware<RedisSessionStore> {
    dotenvy::dotenv().ok();

    let session_secret = env::var("SESSION_SECRET").expect("SESSION_SECRET must be specified");

    let secret_key = Key::from(session_secret.as_bytes());

    SessionMiddleware::new(store, secret_key)
}