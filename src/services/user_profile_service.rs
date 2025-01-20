use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

use crate::domain::entities::profile_tag::ProfileTag;
use crate::domain::entities::user_profile::UserProfile;
use crate::domain::errors::user_profile_error::UserProfileError;
use crate::domain::repositories::user_profile_repo::{UserProfileQueryParams, UserProfileRepository};
use crate::domain::services::user_profile_service::UserProfileService;
use crate::infrastructure::models::user_profile::{UserProfileInsert, UserProfileUpdate};
use crate::infrastructure::repositories::user_profile_repo::PgUserProfileRepository;
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
    #[tracing::instrument(skip(self))]
    async fn create(&self, profile: &UserProfileInsert) -> Result<(), UserProfileError> {
        let mut tx = self.pool.begin().await?;

        PgUserProfileRepository::insert(&mut *tx, profile).await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn get_by_id(&self, profile_id: Snowflake) -> Result<UserProfile, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let profile = PgUserProfileRepository::get_by_id(&mut *conn, profile_id).await?;

        Ok(profile)
    }

    #[tracing::instrument(skip(self))]
    async fn get_by_user_id(&self, user_id: Snowflake) -> Result<UserProfile, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let profile = PgUserProfileRepository::get_by_user_id(&mut *conn, user_id).await?;

        Ok(profile)
    }

    #[tracing::instrument(skip(self))]
    async fn update(&self, id: Snowflake, profile: &UserProfileUpdate) -> Result<(), UserProfileError> {
        let mut tx = self.pool.begin().await?;

        PgUserProfileRepository::update(&mut *tx, id, profile).await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn search(&self, params: &UserProfileQueryParams) -> Result<Vec<UserProfile>, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let profiles = PgUserProfileRepository::search(&mut *conn, params).await?;

        Ok(profiles)
    }

    #[tracing::instrument(skip(self))]
    async fn get_profile_tags(&self, profile_id: Snowflake) -> Result<Vec<ProfileTag>, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let tags = PgUserProfileRepository::get_profile_tags(&mut *conn, profile_id).await?;

        Ok(tags)
    }

    #[tracing::instrument(skip(self))]
    async fn add_pictures(&self, profile_id: Snowflake, picture_hashes: Vec<String>) -> Result<(), UserProfileError> {
        let mut tx = self.pool.begin().await?;

        PgUserProfileRepository::add_pictures(&mut *tx, profile_id, picture_hashes).await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn remove_pictures(
        &self,
        profile_id: Snowflake,
        picture_hashes: Vec<String>,
    ) -> Result<(), UserProfileError> {
        let mut tx = self.pool.begin().await?;

        PgUserProfileRepository::remove_pictures(&mut *tx, profile_id, picture_hashes).await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn add_tag(&self, profile_id: Snowflake, tag_id: Snowflake) -> Result<(), UserProfileError> {
        let mut tx = self.pool.begin().await?;

        PgUserProfileRepository::add_tag(&mut *tx, profile_id, tag_id).await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn remove_tag(&self, profile_id: Snowflake, tag_id: Snowflake) -> Result<(), UserProfileError> {
        let mut tx = self.pool.begin().await?;

        PgUserProfileRepository::remove_tag(&mut *tx, profile_id, tag_id).await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn bulk_add_tags(&self, profile_id: Snowflake, tag_ids: Vec<Snowflake>) -> Result<(), UserProfileError> {
        let mut tx = self.pool.begin().await?;

        PgUserProfileRepository::bulk_add_tags(&mut *tx, profile_id, tag_ids).await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn bulk_remove_tags(&self, profile_id: Snowflake, tag_ids: Vec<Snowflake>) -> Result<(), UserProfileError> {
        let mut tx = self.pool.begin().await?;

        PgUserProfileRepository::bulk_remove_tags(&mut *tx, profile_id, tag_ids).await?;

        tx.commit().await?;

        Ok(())
    }
}
