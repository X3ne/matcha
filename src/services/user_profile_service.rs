use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

use crate::domain::entities::user_profile::UserProfile;
use crate::domain::errors::user_error::UserError;
use crate::domain::repositories::user_profile_repo::UserProfileRepository;
use crate::domain::services::user_profile_service::UserProfileService;
use crate::infrastructure::models::user_profile::UserProfileInsert;
use crate::infrastructure::repositories::user_profile::PgUserProfileRepository;
use crate::shared::types::snowflake::Snowflake;

#[derive(Clone)]
pub struct UserProfileServiceImpl {
    pub pool: Arc<PgPool>,
}

impl UserProfileServiceImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        UserProfileServiceImpl { pool }
    }
}

#[async_trait]
impl UserProfileService for UserProfileServiceImpl {
    async fn create(&self, profile: &UserProfileInsert) -> Result<(), UserError> {
        let mut conn = self.pool.acquire().await?;

        PgUserProfileRepository::insert(&mut *conn, profile).await?;

        Ok(())
    }

    async fn get_by_id(&self, profile_id: Snowflake) -> Result<UserProfile, UserError> {
        let mut conn = self.pool.acquire().await?;

        let profile = PgUserProfileRepository::get_by_id(&mut *conn, profile_id).await?;

        Ok(profile)
    }

    async fn get_by_user_id(&self, user_id: Snowflake) -> Result<UserProfile, UserError> {
        let mut conn = self.pool.acquire().await?;

        let profile = PgUserProfileRepository::get_by_user_id(&mut *conn, user_id).await?;

        Ok(profile)
    }
}
