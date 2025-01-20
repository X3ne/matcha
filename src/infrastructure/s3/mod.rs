pub mod error;

use s3::creds::Credentials;
use s3::error::S3Error;
use s3::region::Region;
use s3::Bucket;

use crate::config::S3Config;
use crate::infrastructure::s3::error::ImageError;

pub struct S3Service {
    bucket: Box<Bucket>,
    config: S3Config,
}

impl S3Service {
    pub async fn new(config: S3Config) -> Result<Self, ImageError> {
        let region = Region::Custom {
            region: config.region.clone(),
            endpoint: config.endpoint.clone(),
        };
        let credentials = Credentials::new(
            Some(&config.access_key_id),
            Some(&config.secret_access_key),
            None,
            None,
            None,
        )?;

        let bucket = Bucket::new(&config.bucket_name, region.clone(), credentials.clone())?.with_path_style();

        Ok(Self { bucket, config })
    }

    pub async fn get_file(&self, key: &str) -> Result<Vec<u8>, ImageError> {
        let res = self.bucket.get_object(key).await.map_err(|e| match e {
            S3Error::HttpFailWithBody(404, ..) => ImageError::ImageNotFound,
            _ => ImageError::S3Error(e),
        })?;

        Ok(Vec::from(res.as_slice()))
    }

    pub async fn upload_file(&self, image: &[u8], content_type: &str, key: &str) -> Result<(), ImageError> {
        let res = self
            .bucket
            .put_object_with_content_type(key, image, content_type)
            .await?;

        if res.status_code() != 200 {
            return Err(ImageError::UploadError);
        }

        Ok(())
    }

    pub async fn delete_file(&self, key: &str) -> Result<(), ImageError> {
        let res = self.bucket.delete_object(key).await?;

        if res.status_code() != 204 {
            return Err(ImageError::DeleteError);
        }

        Ok(())
    }
}
