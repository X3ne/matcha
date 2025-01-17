use actix_web::http::StatusCode;

use crate::infrastructure::opcodes::ErrorCode;
use crate::ApiErrorImpl;

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("Database error")]
    DatabaseError,
    #[error("User does not have a profile")]
    NoProfile,
    #[error("Maximum number of profile images reached")]
    MaxImages,
    #[error("Profile not found")]
    ProfileNotFound,
    #[error("This user already have a profile")]
    UserAlreadyHaveProfile,
}

impl ApiErrorImpl for UserError {
    fn get_codes(&self) -> (StatusCode, ErrorCode) {
        match self {
            UserError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            UserError::NoProfile => (StatusCode::NOT_FOUND, ErrorCode::UnknownProfile),
            UserError::MaxImages => (StatusCode::CONFLICT, ErrorCode::MaxImages),
            UserError::ProfileNotFound => (StatusCode::NOT_FOUND, ErrorCode::UnknownProfile),
            UserError::UserAlreadyHaveProfile => (StatusCode::CONFLICT, ErrorCode::UserAlreadyHaveProfile),
        }
    }
}

impl From<sqlx::Error> for UserError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", e);
        tracing::error!("Database error: {}", e);
        match e {
            sqlx::Error::RowNotFound => UserError::ProfileNotFound,
            sqlx::Error::Database(db_err) => {
                if let Some(constraint) = db_err.constraint() {
                    match constraint {
                        "user_profile_user_id_key" => UserError::UserAlreadyHaveProfile,
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
