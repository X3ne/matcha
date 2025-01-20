use std::io::Read;
use std::str::FromStr;
use std::sync::Arc;

use actix_multipart::form::tempfile::TempFile;
use async_trait::async_trait;
use mime::Mime;
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::domain::repositories::user_profile_repo::UserProfileRepository;
use crate::domain::services::cdn_service::CdnService;
use crate::infrastructure::repositories::user_profile_repo::PgUserProfileRepository;
use crate::infrastructure::s3::error::ImageError;
use crate::infrastructure::s3::S3Service;

#[derive(Clone)]
pub struct CdnServiceImpl {
    pub s3: Arc<S3Service>,
    pub pool: Arc<PgPool>,
}

impl CdnServiceImpl {
    pub fn new(s3: Arc<S3Service>, pool: Arc<PgPool>) -> Self {
        CdnServiceImpl { s3, pool }
    }
}

#[async_trait]
impl CdnService for CdnServiceImpl {
    #[tracing::instrument(skip(self))]
    async fn get_by_hash(&self, hash: &str) -> Result<Vec<u8>, ImageError> {
        self.s3.get_file(hash).await
    }

    #[tracing::instrument(skip(self))]
    async fn upload_file(&self, file: &mut TempFile, path: &str) -> Result<String, ImageError> {
        let mut file_content = Vec::new();

        file.file.read_to_end(&mut file_content).map_err(|e| {
            tracing::error!("Error reading file: {}", e);
            ImageError::ParseError
        })?;

        let content_type = file
            .content_type
            .clone()
            .unwrap_or(Mime::from_str("image/png").map_err(|e| ImageError::InvalidMime(e))?);

        let mut hasher = Sha256::new();
        hasher.update(&file_content);
        let hash_result = hasher.finalize();
        let file_hash = format!("{:x}", hash_result);

        tracing::info!("Uploading file with hash: {}", file_hash);

        self.s3
            .upload_file(
                &file_content,
                &content_type.to_string(),
                &format!("{}/{}", path, file_hash),
            )
            .await?;

        Ok(file_hash)
    }

    async fn delete_file(&self, path: &str) -> Result<(), ImageError> {
        self.s3.delete_file(path).await
    }

    async fn is_picture_hash_used(&self, hash: &str) -> Result<bool, ImageError> {
        let mut conn = self.pool.acquire().await?;

        let is_used = PgUserProfileRepository::is_profile_hash_used(&mut *conn, hash).await?;

        Ok(is_used)
    }
}
