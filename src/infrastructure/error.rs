use std::collections::HashMap;
use std::fmt::Debug;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use apistos::ApiErrorComponent;
use oauth2::error::OAuth2Error;
use regex::Regex;

use crate::domain::errors::auth_error::AuthError;
use crate::domain::errors::user_error::UserError;
use crate::{ApiErrorImpl, ErrorResponse};

#[derive(thiserror::Error, Debug, ApiErrorComponent)]
#[openapi_error(
    status(code = 403),
    status(code = 404),
    status(code = 405, description = "Invalid input"),
    status(code = 409)
)]
#[allow(dead_code)]
pub enum ApiError {
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Internal Server Error")]
    InternalServerError,
    #[error(transparent)]
    SessionError(#[from] actix_session::SessionGetError),
    #[error(transparent)]
    ValidationError(#[from] garde::error::Report),
    #[error("Database Error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("OAuth2 Error: {0}")]
    OAuth2Error(#[from] OAuth2Error),
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    UserError(#[from] UserError),
}

impl ApiErrorImpl for ApiError {
    fn get_codes(&self) -> (StatusCode, &str) {
        match self {
            ApiError::BadRequest(..) => (StatusCode::BAD_REQUEST, "bad_request"),
            ApiError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "internal_server_error"),
            ApiError::SessionError(..) => (StatusCode::INTERNAL_SERVER_ERROR, "session_error"),
            ApiError::ValidationError(..) => (StatusCode::BAD_REQUEST, "validation_error"),
            ApiError::DatabaseError(..) => (StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
            ApiError::OAuth2Error(..) => (StatusCode::INTERNAL_SERVER_ERROR, "oauth2_error"),
            ApiError::AuthError(err) => err.get_codes(),
            ApiError::UserError(err) => err.get_codes(),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, code) = self.get_codes();
        let message = self.to_string();

        tracing::error!("{}", self);

        fn parse_errors(description: &str) -> HashMap<String, String> {
            let mut errors_map = HashMap::new();

            let re = Regex::new(r"(\w+):\s*([^\[]+)\[.*?\]").unwrap();

            for cap in re.captures_iter(description) {
                errors_map.insert(
                    cap[1].to_string(),
                    cap[2]
                        .to_string()
                        .replace("Validation error: ", "invalid_")
                        .trim()
                        .to_lowercase(),
                );
            }

            errors_map
        }

        let error_response = match self {
            ApiError::ValidationError(_) => {
                let description = self.to_string();
                ErrorResponse {
                    code,
                    message: "Validation error".to_string(),
                    details: None,
                    form_errors: Some(parse_errors(&description)),
                }
            }
            _ => ErrorResponse {
                code,
                message,
                details: None,
                form_errors: None,
            },
        };

        HttpResponse::build(status).json(error_response)
    }
}
