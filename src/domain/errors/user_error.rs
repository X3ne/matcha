use actix_web::http::StatusCode;

use crate::infrastructure::opcodes::ErrorCode;
use crate::ApiErrorImpl;

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("Database error")]
    DatabaseError,
    #[error("Profile not found")]
    UserNotFound,
}

impl ApiErrorImpl for UserError {
    fn get_codes(&self) -> (StatusCode, ErrorCode) {
        match self {
            UserError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            UserError::UserNotFound => (StatusCode::NOT_FOUND, ErrorCode::UnknownUser),
        }
    }
}

impl From<sqlx::Error> for UserError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", e);
        match e {
            sqlx::Error::RowNotFound => UserError::UserNotFound,
            sqlx::Error::Database(db_err) => {
                if let Some(constraint) = db_err.constraint() {
                    match constraint {
                        _ => UserError::DatabaseError,
                    }
                } else {
                    UserError::DatabaseError
                }
            }
            _ => UserError::DatabaseError,
        }
    }
}
