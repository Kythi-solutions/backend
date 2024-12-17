use crate::database::repository::user::UserRepository;
use actix_web::{post, web, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CredentialLogin {
    username: String,
    password: String
}

#[post("/auth/credential")]
async fn credential_auth(user: web::Data<UserRepository>, form: web::Json<CredentialLogin>) -> impl Responder {
    "meow"
}