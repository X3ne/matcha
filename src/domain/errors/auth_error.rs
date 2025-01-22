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
    #[error("Redis error")]
    RedisError,
    #[error("Password is required")]
    PasswordRequired,
    #[error("Error hashing password")]
    PasswordHashError,
    #[error("Error sending mail")]
    MailError,
    #[error("User not found")]
    UserNotFound,
    #[error("An user with this email already exists")]
    UserEmailAlreadyExists,
    #[error("An user with this username already exists")]
    UserUsernameAlreadyExists,
    #[error("Invalid reset token")]
    InvalidResetToken,
}

impl ApiErrorImpl for AuthError {
    fn get_codes(&self) -> (StatusCode, ErrorCode) {
        match self {
            AuthError::Unauthorized => (StatusCode::UNAUTHORIZED, ErrorCode::Unauthorized),
            AuthError::AccountNotActivated => (StatusCode::UNAUTHORIZED, ErrorCode::AccountNotActivated),
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, ErrorCode::InvalidCredentials),
            AuthError::OAuth2Error(..) => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            AuthError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            AuthError::RedisError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            AuthError::PasswordRequired => (StatusCode::BAD_REQUEST, ErrorCode::InvalidCredentials),
            AuthError::PasswordHashError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            AuthError::MailError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            AuthError::UserNotFound => (StatusCode::NOT_FOUND, ErrorCode::UnknownUser),
            AuthError::UserEmailAlreadyExists => (StatusCode::CONFLICT, ErrorCode::UserEmailAlreadyExists),
            AuthError::UserUsernameAlreadyExists => (StatusCode::CONFLICT, ErrorCode::UserUsernameAlreadyExists),
            AuthError::InvalidResetToken => (StatusCode::BAD_REQUEST, ErrorCode::InvalidResetToken),
        }
    }
}

impl From<sqlx::Error> for AuthError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", e);
        match e {
            sqlx::Error::RowNotFound => AuthError::UserNotFound,
            sqlx::Error::Database(db_err) => {
                if let Some(constraint) = db_err.constraint() {
                    match constraint {
                        "user_email_key" => AuthError::UserEmailAlreadyExists,
                        "user_username_key" => AuthError::UserUsernameAlreadyExists,
                        _ => AuthError::DatabaseError,
                    }
                } else {
                    AuthError::DatabaseError
                }
            }
            _ => AuthError::DatabaseError,
        }
    }
}

impl From<redis::RedisError> for AuthError {
    fn from(e: redis::RedisError) -> Self {
        tracing::error!("Redis error: {}", e);
        match e.kind() {
            redis::ErrorKind::TypeError => AuthError::InvalidResetToken,
            _ => AuthError::RedisError,
        }
    }
}
