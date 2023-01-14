use actix_api_error_derive::AsApiError;
use serde::{Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct ApiError {
    pub kind: &'static str,
    pub code: u16,
    pub messages: HashMap<String, String>,
}

impl ApiError {
    /// Returns a new `ApiError` with the given kind and message.
    pub fn to_json(&self) -> serde_json::error::Result<String>  {
        serde_json::to_string(&self)
    }
}

pub trait AsApiError {
    fn as_api_error(&self) -> ApiError;
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} : {:?}", self.kind, self.messages)
    }
}

#[cfg(feature = "actix")]
use actix_web::http::StatusCode;
#[cfg(feature = "actix")]
impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.code
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).json(self.messages.clone())
    }
}

#[derive(AsApiError)]
//#[po_directory = ".po"]
pub enum ErrorEn {
    #[error(code = 404, msg_id = "invalid_password")]
    InvalidPassword,
    #[error(code = 404, msg_id = "invalid_id")]
    InvalidId(u32),
}

#[test]
fn default() {
    let e = ErrorEn::InvalidPassword;
    println!("Error::to_code() = {:?}", e.as_api_error());
    let e = ErrorEn::InvalidId(42);
    println!("Error::to_code() = {:?}", e.as_api_error());
    
}