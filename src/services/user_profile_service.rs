use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

use crate::domain::entities::user_profile::UserProfile;
use crate::domain::errors::user_error::UserError;
use crate::domain::repositories::user_profile_repo::{UserProfileQueryParams, UserProfileRepository};
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
        let mut tx = self.pool.begin().await?;

        PgUserProfileRepository::insert(&mut *tx, profile).await?;

        tx.commit().await?;

        Ok(())
    }

    async fn get_by_id(&self, profile_id: Snowflake) -> Result<UserProfile, UserError> {
        let mut conn = self.pool.acquire().await?;

        let profile = PgUserProfileRepository::get_by_id(&mut *conn, profile_id).await?;

        Ok(profile)
    }

    async fn get_by_user_id(&self, user_id: Snowflake) -> Result<UserProfile, UserError> {
        let mut conn = self.pool.acquire().await?;

        let profile = PgUserProfileRepository::get_by_user_id(&mut *conn, user_id)
            .await
            .map_err(|e| {
                tracing::error!("Error getting user profile by user id: {:?}", e);
                match e {
                    sqlx::Error::RowNotFound => UserError::NoProfile,
                    _ => UserError::DatabaseError,
                }
            })?;

        Ok(profile)
    }

    async fn search(&self, params: UserProfileQueryParams) -> Result<Vec<UserProfile>, UserError> {
        let mut conn = self.pool.acquire().await?;

        let profiles = PgUserProfileRepository::search(&mut *conn, params).await?;

        Ok(profiles)
    }
}
