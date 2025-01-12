use actix_web::http::StatusCode;

use crate::ApiErrorImpl;

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("Database error")]
    DatabaseError,
}

impl ApiErrorImpl for UserError {
    fn get_codes(&self) -> (StatusCode, &str) {
        match self {
            UserError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "database_error"),
        }
    }
}

impl From<sqlx::Error> for UserError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", e);
        UserError::DatabaseError
    }
}
