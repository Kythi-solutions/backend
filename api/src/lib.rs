use actix_web::middleware::from_fn;
use actix_web::{App, HttpServer};
use actix_web::web::Data;

use database::repository::user::UserRepository;

mod app;
mod infra;

pub async fn run() -> std::io::Result<()> {
    let db = database::pool::create().await.expect("Failed to create database pool");
    let redis = database::redis::create().await.expect("Failed to create redis connection");
    
    let user_repo = Data::new(UserRepository::new(std::sync::Arc::from(db.clone())));

    // TODO: SSL impl
    HttpServer::new(move || {
        App::new()
            .wrap(infra::middleware::session::create(redis.clone()).clone())
            .wrap(from_fn(infra::middleware::auth::check_auth_mw))
            .app_data(user_repo.clone())
    })
     // TODO: make the ip and port also not hardcoded
    .bind(("127.0.0.1", 4001))?
    .run()
    .await   
}