use actix_web::http::StatusCode;

use crate::infrastructure::opcodes::ErrorCode;
use crate::ApiErrorImpl;

#[derive(Debug, thiserror::Error)]
pub enum MessageError {
    #[error("Database error")]
    DatabaseError,
    #[error("Message not found")]
    MessageNotFound,
}

impl ApiErrorImpl for MessageError {
    fn get_codes(&self) -> (StatusCode, ErrorCode) {
        match self {
            MessageError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            MessageError::MessageNotFound => (StatusCode::NOT_FOUND, ErrorCode::UnknownMessage),
        }
    }
}

impl From<sqlx::Error> for MessageError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", e);
        match e {
            sqlx::Error::RowNotFound => MessageError::MessageNotFound,
            sqlx::Error::Database(db_err) => {
                if let Some(constraint) = db_err.constraint() {
                    match constraint {
                        _ => MessageError::DatabaseError,
                    }
                } else {
                    MessageError::DatabaseError
                }
            }
            _ => MessageError::DatabaseError,
        }
    }
}
