use actix_web::{App, HttpServer};
use actix_web::web::Data;

use crate::state::GlobalState;
use database;
use database::repository::user::UserRepository;

pub mod state;
mod typings;
mod telemetry;
mod controller;

pub async fn run() -> std::io::Result<()> {
    let db = database::pool::create().await.expect("Failed to create database pool");

    // TODO: make version not hardcoded lol
    let state = Data::new(GlobalState {
        version: "1.0".to_string(),
        db: db.clone(),
    });

    let user_repo = Data::new(UserRepository::new(std::sync::Arc::from(db.clone())));

    // TODO: SSL impl
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(user_repo.clone())
            // .service(hello)
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello))
    })
     // TODO: make the ip and port also not hardcoded
    .bind(("127.0.0.1", 4001))?
    .run()
    .await   
}