use crate::infrastructure::opcodes::ErrorCode;
use crate::ApiErrorImpl;
use actix_web::http::StatusCode;
use s3::creds::error::CredentialsError;
use s3::error::S3Error;

#[derive(Debug, thiserror::Error)]
pub enum ImageError {
    #[error("Invalid image format")]
    InvalidImageFormat,
    #[error("Credentials error")]
    CredentialsError(#[from] CredentialsError),
    #[error("S3 error")]
    S3Error(#[from] S3Error),
    #[error("Error parsing image")]
    ParseError,
    #[error("Error uploading image")]
    UploadError,
    #[error("Image not found")]
    ImageNotFound,
    #[error("Error encoding image")]
    EncodingError,
    #[error("Invalid Mime type")]
    InvalidMime(#[from] mime::FromStrError),
}

impl ApiErrorImpl for ImageError {
    fn get_codes(&self) -> (StatusCode, ErrorCode) {
        match self {
            ImageError::InvalidImageFormat => (StatusCode::BAD_REQUEST, ErrorCode::Default),
            ImageError::CredentialsError(_) => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            ImageError::S3Error(_) => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            ImageError::ParseError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            ImageError::UploadError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            ImageError::ImageNotFound => (StatusCode::NOT_FOUND, ErrorCode::Default),
            ImageError::EncodingError => (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::Default),
            ImageError::InvalidMime(_) => (StatusCode::BAD_REQUEST, ErrorCode::Default),
        }
    }
}
