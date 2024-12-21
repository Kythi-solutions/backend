use std::{collections::HashMap, num::{NonZero, NonZeroU16}};

use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ResponseCode(NonZeroU16);

impl Into<ResponseCode> for StatusCode {
    fn into(self) -> ResponseCode {
        ResponseCode(NonZero::new(self.as_u16()).unwrap())
    }
}

#[derive(Serialize, Debug)]
pub struct Response {
    code: ResponseCode,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<HashMap<String, serde_json::Value>>,
}

impl Response {
    pub fn new(code: StatusCode, message: String, error: Option<String>, data: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self {
            code: code.into(),
            message,
            error,
            data
        }
    }

    pub fn internal_error(error: String) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR.into(),
            message: "An internal server error has occurred".to_string(),
            error: Some(error),
            data: None
        }
    }

    pub fn unauthorized() -> Self {
        Self {
            code: StatusCode::UNAUTHORIZED.into(),
            message: "You are not authorized to make this request".to_string(),
            error: None,
            data: None
        }
    }
}
