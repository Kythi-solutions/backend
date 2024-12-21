use actix_session::SessionExt;
use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, error, http::StatusCode, middleware::Next, web, Error, HttpResponse, ResponseError};
use derive_more::derive::Display;
use serde::Serialize;
use serde_json::json;

use crate::infra::handler::response::Response;

pub async fn check_auth_mw(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let jwt_token = req.get_session().get::<String>("Authorization")?;

    if let None = jwt_token {
        return Err(error::ErrorUnauthorized(json!(Response::unauthorized())))
    }
   
    next.call(req).await
}
 