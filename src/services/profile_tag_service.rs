use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::profile_tag::ProfileTag;
use crate::domain::errors::profile_tag_error::ProfileTagError;
use crate::domain::repositories::profile_tag_repository::ProfileTagRepository;
use crate::domain::services::profile_tag_service::ProfileTagService;
use crate::infrastructure::repositories::profile_tag_repo::PgProfileTagRepository;
use crate::shared::types::snowflake::Snowflake;

#[derive(Clone)]
pub struct ProfileTagServiceImpl {
    pub pool: Arc<PgPool>,
}

impl ProfileTagServiceImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        ProfileTagServiceImpl { pool }
    }
}

#[async_trait]
impl ProfileTagService for ProfileTagServiceImpl {
    async fn get_by_id(&self, tag_id: Snowflake) -> Result<ProfileTag, ProfileTagError> {
        let mut conn = self.pool.acquire().await?;

        let tag = PgProfileTagRepository::get_by_id(&mut *conn, tag_id).await?;

        Ok(tag)
    }

    async fn get_by_ids(&self, tag_ids: Vec<Snowflake>) -> Result<Vec<ProfileTag>, ProfileTagError> {
        let mut conn = self.pool.acquire().await?;

        let tags = PgProfileTagRepository::get_by_ids(&mut *conn, tag_ids).await?;

        Ok(tags)
    }

    async fn get_all(&self) -> Result<Vec<ProfileTag>, ProfileTagError> {
        let mut conn = self.pool.acquire().await?;

        let tags = PgProfileTagRepository::get_all(&mut *conn).await?;

        Ok(tags)
    }
}
