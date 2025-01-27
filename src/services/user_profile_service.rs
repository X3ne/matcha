use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::profile_tag::ProfileTag;
use crate::domain::entities::user_profile::UserProfile;
use crate::domain::errors::user_profile_error::UserProfileError;
use crate::domain::repositories::user_profile_repo::{UserProfileQueryParams, UserProfileRepository};
use crate::domain::services::user_profile_service::UserProfileService;
use crate::infrastructure::models::user_profile::{UserProfileInsert, UserProfileUpdate};
use crate::infrastructure::repositories::user_profile_repo::PgUserProfileRepository;
use crate::shared::types::snowflake::Snowflake;
use crate::shared::types::user_profile::{Gender, Orientation};
use crate::shared::utils::fame::FameCalculator;

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
    async fn search(
        &self,
        params: &UserProfileQueryParams,
        current_profile_id: Snowflake,
    ) -> Result<Vec<UserProfile>, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let profiles = PgUserProfileRepository::search(&mut *conn, params, current_profile_id).await?;

        Ok(profiles)
    }

    #[tracing::instrument(skip(self))]
    async fn recommend(
        &self,
        user_id: Snowflake,
        location: geo_types::Geometry<f64>,
        radius_km: f64,
        gender: Gender,
        orientation: Orientation,
        min_age: u8,
        max_age: u8,
    ) -> Result<Vec<UserProfile>, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let profiles = PgUserProfileRepository::recommend(
            &mut *conn,
            user_id,
            location,
            radius_km,
            gender,
            orientation,
            min_age,
            max_age,
        )
        .await?;

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

    #[tracing::instrument(skip(self))]
    async fn add_like(&self, profile: &UserProfile, liked_profile_id: Snowflake) -> Result<(), UserProfileError> {
        let mut tx = self.pool.begin().await?;

        let liked_profile = PgUserProfileRepository::get_by_id(&mut *tx, liked_profile_id).await?;

        PgUserProfileRepository::add_like(&mut *tx, profile.id, liked_profile_id).await?;

        let fame_multiplier = FameCalculator::calculate_fame_multiplier(profile.rating);
        let fame_increase = FameCalculator::calculate_fame(liked_profile.rating, fame_multiplier);

        PgUserProfileRepository::increase_fame_rating(&mut *tx, liked_profile_id, fame_increase).await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn remove_like(&self, profile_id: Snowflake, liked_profile_id: Snowflake) -> Result<(), UserProfileError> {
        let mut tx = self.pool.begin().await?;

        let liked_profile = PgUserProfileRepository::get_by_id(&mut *tx, liked_profile_id).await?;

        PgUserProfileRepository::remove_like(&mut *tx, profile_id, liked_profile_id).await?;

        let fame_multiplier = FameCalculator::calculate_fame_multiplier(liked_profile.rating);
        let fame_decrease = FameCalculator::calculate_fame(liked_profile.rating, fame_multiplier);

        PgUserProfileRepository::decrease_fame_rating(&mut *tx, liked_profile_id, fame_decrease).await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn is_profile_liked(
        &self,
        profile_id: Snowflake,
        liked_profile_id: Snowflake,
    ) -> Result<bool, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let is_liked = PgUserProfileRepository::is_like_exists(&mut *conn, profile_id, liked_profile_id).await?;

        Ok(is_liked)
    }

    #[tracing::instrument(skip(self))]
    async fn is_profile_matched(
        &self,
        profile_id: Snowflake,
        matched_profile_id: Snowflake,
    ) -> Result<bool, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let is_matched = PgUserProfileRepository::is_match_exists(&mut *conn, profile_id, matched_profile_id).await?;

        Ok(is_matched)
    }

    #[tracing::instrument(skip(self))]
    async fn get_my_likes(&self, profile_id: Snowflake) -> Result<Vec<UserProfile>, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let profiles = PgUserProfileRepository::get_my_likes(&mut *conn, profile_id).await?;

        Ok(profiles)
    }

    #[tracing::instrument(skip(self))]
    async fn get_profile_likes(&self, profile_id: Snowflake) -> Result<Vec<UserProfile>, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let profiles = PgUserProfileRepository::get_profile_likes(&mut *conn, profile_id).await?;

        Ok(profiles)
    }

    #[tracing::instrument(skip(self))]
    async fn get_matches(&self, profile_id: Snowflake) -> Result<Vec<UserProfile>, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let profiles = PgUserProfileRepository::get_matches(&mut *conn, profile_id).await?;

        Ok(profiles)
    }

    #[tracing::instrument(skip(self))]
    async fn view_profile(&self, profile_id: Snowflake, viewed_profile_id: Snowflake) -> Result<(), UserProfileError> {
        let mut tx = self.pool.begin().await?;

        PgUserProfileRepository::view_profile(&mut *tx, profile_id, viewed_profile_id).await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn get_viewers(&self, profile_id: Snowflake) -> Result<Vec<UserProfile>, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let profiles = PgUserProfileRepository::get_viewers(&mut *conn, profile_id).await?;

        Ok(profiles)
    }
}
