use actix_web::http::StatusCode;

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
    fn get_codes(&self) -> (StatusCode, &str) {
        match self {
            AuthError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            AuthError::AccountNotActivated => (StatusCode::UNAUTHORIZED, "account_not_activated"),
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "invalid_credentials"),
            AuthError::OAuth2Error(..) => (StatusCode::INTERNAL_SERVER_ERROR, "oauth2_error"),
            AuthError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
            AuthError::PasswordRequired => (StatusCode::BAD_REQUEST, "password_required"),
            AuthError::PasswordHashError => (StatusCode::INTERNAL_SERVER_ERROR, "hash_error"),
            AuthError::MailError => (StatusCode::INTERNAL_SERVER_ERROR, "mail_error"),
        }
    }
}

impl From<sqlx::Error> for AuthError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", e);
        AuthError::DatabaseError
    }
}
