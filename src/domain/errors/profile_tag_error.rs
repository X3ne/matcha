use actix_web::http::StatusCode;

use crate::infrastructure::opcodes::ErrorCode;
use crate::ApiErrorImpl;

#[derive(Debug, thiserror::Error)]
pub enum ProfileTagError {
    #[error("Database error")]
    DatabaseError,
    #[error("Tag not found")]
    TagNotFound,
    #[error("Tag already exists")]
    TagAlreadyExists,
}

impl ApiErrorImpl for ProfileTagError {
    fn get_codes(&self) -> (StatusCode, ErrorCode) {
        match self {
            ProfileTagError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            ProfileTagError::TagNotFound => (StatusCode::NOT_FOUND, ErrorCode::UnknownTag),
            ProfileTagError::TagAlreadyExists => (StatusCode::CONFLICT, ErrorCode::TagAlreadyExists),
        }
    }
}

impl From<sqlx::Error> for ProfileTagError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", e);
        match e {
            sqlx::Error::RowNotFound => ProfileTagError::TagNotFound,
            sqlx::Error::Database(db_err) => {
                if let Some(constraint) = db_err.constraint() {
                    match constraint {
                        "profile_tag_name_key" => ProfileTagError::TagAlreadyExists,
                        _ => ProfileTagError::DatabaseError,
                    }
                } else {
                    ProfileTagError::DatabaseError
                }
            }
            _ => ProfileTagError::DatabaseError,
        }
    }
}
