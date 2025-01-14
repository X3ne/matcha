use actix_web::http::StatusCode;

use crate::infrastructure::opcodes::ErrorCode;
use crate::ApiErrorImpl;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Account not activated")]
    AccountNotActivated,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("OAuth2 error: {0}")]
    OAuth2Error(#[from] oauth2::error::OAuth2Error),
    #[error("Database error")]
    DatabaseError,
    #[error("Password is required")]
    PasswordRequired,
    #[error("Error hashing password")]
    PasswordHashError,
    #[error("Error sending mail")]
    MailError,
}

impl ApiErrorImpl for AuthError {
    fn get_codes(&self) -> (StatusCode, ErrorCode) {
        match self {
            AuthError::Unauthorized => (StatusCode::UNAUTHORIZED, ErrorCode::Unauthorized),
            AuthError::AccountNotActivated => (StatusCode::UNAUTHORIZED, ErrorCode::AccountNotActivated),
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, ErrorCode::InvalidCredentials),
            AuthError::OAuth2Error(..) => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            AuthError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            AuthError::PasswordRequired => (StatusCode::BAD_REQUEST, ErrorCode::InvalidCredentials),
            AuthError::PasswordHashError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            AuthError::MailError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
        }
    }
}

impl From<sqlx::Error> for AuthError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", e);
        AuthError::DatabaseError
    }
}
