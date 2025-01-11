use std::collections::HashMap;
use std::fmt::Display;

use actix_web::http::StatusCode;
use actix_web::ResponseError;
use serde::Serialize;

mod domain;
mod infrastructure;
mod presentation;
mod services;
mod shared;

pub use infrastructure::config;
pub use infrastructure::server;
pub use infrastructure::tracing;

// TODO: move this
#[derive(Serialize, Debug, Clone)]
pub struct ErrorDetails {
    pub message: String,
    pub code: String,
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse<'a> {
    pub code: &'a str,
    pub message: String,
    pub details: Option<Vec<ErrorDetails>>,
    pub form_errors: Option<HashMap<String, String>>,
}

impl ResponseError for ErrorResponse<'_> {}

impl Display for ErrorResponse<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ErrorResponse {{ code: {}, message: {}, details: {:?}, form_errors: {:?} }}",
            self.code, self.message, self.details, self.form_errors
        )
    }
}

pub trait ApiErrorImpl {
    fn get_codes(&self) -> (StatusCode, &str);
}
