use actix_web::{post, web};
use serde::{Deserialize, Serialize};
use crate::database::repository::user::UserRepository;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CredentialLogin {
    username: String,
    password: String
}

#[post("/auth/credential")]
async fn credential_auth(user: web::Data<UserRepository>, form: CredentialLogin) -> impl Responder {
    let find_user = user.by_username(form.username).await;



    1
}