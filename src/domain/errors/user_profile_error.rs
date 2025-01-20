use actix_web::http::StatusCode;

use crate::infrastructure::opcodes::ErrorCode;
use crate::ApiErrorImpl;

#[derive(Debug, thiserror::Error)]
pub enum UserProfileError {
    #[error("Database error")]
    DatabaseError,
    #[error("User does not have a profile")]
    NoProfile,
    #[error("Maximum number of profile images reached")]
    MaxImages,
    #[error("Invalid image offset")]
    InvalidImageOffset,
    #[error("Profile not found")]
    ProfileNotFound,
    #[error("This user already have a profile")]
    UserAlreadyHaveProfile,
    #[error("Profile already has this tag")]
    ProfileAlreadyHasTag,
    #[error("Cannot delete avatar")]
    CannotDeleteAvatar,
}

impl ApiErrorImpl for UserProfileError {
    fn get_codes(&self) -> (StatusCode, ErrorCode) {
        match self {
            UserProfileError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            UserProfileError::NoProfile => (StatusCode::NOT_FOUND, ErrorCode::UnknownProfile),
            UserProfileError::MaxImages => (StatusCode::CONFLICT, ErrorCode::MaxImages),
            UserProfileError::InvalidImageOffset => (StatusCode::BAD_REQUEST, ErrorCode::InvalidImageOffset),
            UserProfileError::ProfileNotFound => (StatusCode::NOT_FOUND, ErrorCode::UnknownProfile),
            UserProfileError::UserAlreadyHaveProfile => (StatusCode::CONFLICT, ErrorCode::UserAlreadyHaveProfile),
            UserProfileError::ProfileAlreadyHasTag => (StatusCode::CONFLICT, ErrorCode::ProfileAlreadyHasTag),
            UserProfileError::CannotDeleteAvatar => (StatusCode::BAD_REQUEST, ErrorCode::CannotDeleteAvatar),
        }
    }
}

impl From<sqlx::Error> for UserProfileError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", e);
        match e {
            sqlx::Error::RowNotFound => UserProfileError::ProfileNotFound,
            sqlx::Error::Database(db_err) => {
                if let Some(constraint) = db_err.constraint() {
                    match constraint {
                        "user_profile_user_id_key" => UserProfileError::UserAlreadyHaveProfile,
                        "join_user_profile_tag_user_profile_id_profile_tag_id_key" => {
                            UserProfileError::ProfileAlreadyHasTag
                        }
                        _ => UserProfileError::DatabaseError,
                    }
                } else {
                    UserProfileError::DatabaseError
                }
            }
            _ => UserProfileError::DatabaseError,
        }
    }
}
