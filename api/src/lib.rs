use actix_web::{App, HttpServer};
use actix_web::web::Data;

use crate::state::GlobalState;
use database;

pub mod state;
mod typings;

pub async fn run() -> std::io::Result<()> {
    let db = database::pool::create().await.expect("Failed to create database pool");


    let state = Data::new(GlobalState {
        version: "1.0".to_string(),
        db,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            // .service(hello)
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 4001))?
    .run()
    .await   
}