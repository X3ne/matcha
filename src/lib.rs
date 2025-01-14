use std::collections::BTreeMap;
use std::fmt::Display;

use actix_web::http::StatusCode;
use actix_web::ResponseError;
use serde::Serialize;

mod domain;
mod infrastructure;
mod presentation;
mod services;
mod shared;

use crate::infrastructure::opcodes::ErrorCode;
pub use infrastructure::config;
pub use infrastructure::server;
pub use infrastructure::tracing;

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub code: u32,
    pub message: String,
    pub errors: BTreeMap<String, ErrorDetails>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ErrorDetails {
    _errors: Vec<ErrorItem>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ErrorItem {
    message: String,
}

impl ResponseError for ErrorResponse {}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ErrorResponse {{ code: {}, message: {}, errors: {:?} }}",
            self.code, self.message, self.errors
        )
    }
}

pub trait ApiErrorImpl {
    fn get_codes(&self) -> (StatusCode, ErrorCode);
}
