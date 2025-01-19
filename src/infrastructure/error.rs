use std::collections::BTreeMap;
use std::fmt::Debug;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use apistos::ApiErrorComponent;
use oauth2::error::OAuth2Error;

use crate::domain::errors::auth_error::AuthError;
use crate::domain::errors::profile_tag_error::ProfileTagError;
use crate::domain::errors::user_error::UserError;
use crate::domain::errors::user_profile_error::UserProfileError;
use crate::infrastructure::opcodes::ErrorCode;
use crate::infrastructure::s3::error::ImageError;
use crate::{ApiErrorImpl, ErrorDetails, ErrorItem, ErrorResponse};
use crate::infrastructure::services::iploc::error::IpLocError;

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
    #[error("Only images are allowed")]
    OnlyImagesAllowed,
    #[error(transparent)]
    IpLocError(#[from] IpLocError),
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    UserError(#[from] UserError),
    #[error(transparent)]
    UserProfileError(#[from] UserProfileError),
    #[error(transparent)]
    ImageError(#[from] ImageError),
    #[error(transparent)]
    ProfileTagError(#[from] ProfileTagError),
}

impl ApiErrorImpl for ApiError {
    fn get_codes(&self) -> (StatusCode, ErrorCode) {
        match self {
            ApiError::BadRequest(..) => (StatusCode::BAD_REQUEST, ErrorCode::InvalidFormBody),
            ApiError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            ApiError::SessionError(..) => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::UnknownSession),
            ApiError::ValidationError(..) => (StatusCode::BAD_REQUEST, ErrorCode::InvalidFormBody),
            ApiError::DatabaseError(..) => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            ApiError::OAuth2Error(..) => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            ApiError::OnlyImagesAllowed => (StatusCode::BAD_REQUEST, ErrorCode::OnlyImagesAllowed),
            ApiError::IpLocError(err) => err.get_codes(),
            ApiError::AuthError(err) => err.get_codes(),
            ApiError::UserError(err) => err.get_codes(),
            ApiError::UserProfileError(err) => err.get_codes(),
            ApiError::ImageError(err) => err.get_codes(),
            ApiError::ProfileTagError(err) => err.get_codes(),
        }
    }
}

impl From<&garde::Report> for ErrorResponse {
    fn from(report: &garde::Report) -> Self {
        let mut errors = BTreeMap::new();

        for (path, error) in report.iter() {
            let field = path.to_string();
            let message = error.message().to_string();

            errors
                .entry(field)
                .or_insert_with(|| ErrorDetails { _errors: vec![] })
                ._errors
                .push(ErrorItem { message });
        }

        ErrorResponse {
            code: ErrorCode::InvalidFormBody as u32,
            errors,
            message: ErrorCode::message(&ErrorCode::InvalidFormBody).to_string(),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, code) = self.get_codes();

        tracing::error!("{}", self);

        let error_response = match self {
            ApiError::ValidationError(e) => e.into(),
            _ => ErrorResponse {
                code: code.clone() as u32,
                message: ErrorCode::message(&code).to_string(),
                errors: BTreeMap::new(),
            },
        };

        HttpResponse::build(status).json(error_response)
    }
}
