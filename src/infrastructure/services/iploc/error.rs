use actix_web::http::StatusCode;

use crate::infrastructure::opcodes::ErrorCode;
use crate::ApiErrorImpl;

#[derive(Debug, thiserror::Error)]
pub enum IpLocError {
    #[error("Http error")]
    HttpError(#[from] reqwest::Error),
}

impl ApiErrorImpl for IpLocError {
    fn get_codes(&self) -> (StatusCode, ErrorCode) {
        match self {
            IpLocError::HttpError(..) => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
        }
    }
}
