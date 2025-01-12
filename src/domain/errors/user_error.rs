use actix_web::http::StatusCode;

use crate::ApiErrorImpl;

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("Database error")]
    DatabaseError,
    #[error("User does not have a profile")]
    NoProfile,
}

impl ApiErrorImpl for UserError {
    fn get_codes(&self) -> (StatusCode, &str) {
        match self {
            UserError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
            UserError::NoProfile => (StatusCode::NOT_FOUND, "no_profile"),
        }
    }
}

impl From<sqlx::Error> for UserError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", e);
        UserError::DatabaseError
    }
}
