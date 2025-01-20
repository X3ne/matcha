use actix_multipart::form::tempfile::TempFile;
use async_trait::async_trait;

use crate::infrastructure::s3::error::ImageError;

#[async_trait]
pub trait CdnService: 'static + Sync + Send {
    async fn get_by_hash(&self, hash: &str) -> Result<Vec<u8>, ImageError>;
    async fn upload_file(&self, file: &mut TempFile, path: &str) -> Result<String, ImageError>;
    async fn delete_file(&self, path: &str) -> Result<(), ImageError>;
    async fn is_picture_hash_used(&self, hash: &str) -> Result<bool, ImageError>;
}
