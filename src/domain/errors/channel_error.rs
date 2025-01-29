use actix_web::http::StatusCode;

use crate::infrastructure::opcodes::ErrorCode;
use crate::ApiErrorImpl;

#[derive(Debug, thiserror::Error)]
pub enum ChannelError {
    #[error("Database error")]
    DatabaseError,
    #[error("Channel not found")]
    ChannelNotFound,
    #[error("Channel already exists")]
    ChannelAlreadyExists,
    #[error("Not a channel participant")]
    NotChannelParticipant,
}

impl ApiErrorImpl for ChannelError {
    fn get_codes(&self) -> (StatusCode, ErrorCode) {
        match self {
            ChannelError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            ChannelError::ChannelNotFound => (StatusCode::NOT_FOUND, ErrorCode::UnknownChannel),
            ChannelError::ChannelAlreadyExists => (StatusCode::CONFLICT, ErrorCode::ChannelAlreadyExists),
            ChannelError::NotChannelParticipant => (StatusCode::FORBIDDEN, ErrorCode::NotChannelParticipant),
        }
    }
}

impl From<sqlx::Error> for ChannelError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", e);
        match e {
            sqlx::Error::RowNotFound => ChannelError::ChannelNotFound,
            sqlx::Error::Database(db_err) => {
                if let Some(constraint) = db_err.constraint() {
                    match constraint {
                        _ => ChannelError::DatabaseError,
                    }
                } else {
                    ChannelError::DatabaseError
                }
            }
            _ => ChannelError::DatabaseError,
        }
    }
}
