use std::sync::Arc;

use crate::domain::constants::DISLIKED_PROFILE_TTL;
use crate::domain::entities::profile_tag::ProfileTag;
use crate::domain::entities::user_profile::UserProfile;
use crate::domain::errors::user_profile_error::UserProfileError;
use crate::domain::repositories::chat::channel_repository::ChannelRepository;
use crate::domain::repositories::user_profile_repo::{UserProfileQueryParams, UserProfileRepository};
use crate::domain::services::user_profile_service::UserProfileService;
use crate::infrastructure::models::chat::ChannelInsert;
use crate::infrastructure::models::user_profile::{UserProfileInsert, UserProfileUpdate};
use crate::infrastructure::repositories::chat::channel_repo::PgChannelRepository;
use crate::infrastructure::repositories::user_profile_repo::PgUserProfileRepository;
use crate::shared::types::snowflake::Snowflake;
use crate::shared::types::user_profile::{Gender, Orientation};
use crate::shared::utils::fame::FameCalculator;
use async_trait::async_trait;
use redis::AsyncCommands;
use sqlx::PgPool;

#[derive(Clone)]
pub struct UserProfileServiceImpl {
    pub pool: Arc<PgPool>,
    pub redis: Arc<redis::Client>,
}

impl UserProfileServiceImpl {
    pub fn new(pool: Arc<PgPool>, redis: Arc<redis::Client>) -> Self {
        UserProfileServiceImpl { pool, redis }
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
        excluded_profile_ids: Vec<Snowflake>,
    ) -> Result<Vec<UserProfile>, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let profiles = PgUserProfileRepository::search(&mut *conn, params, excluded_profile_ids).await?;

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
        birth_date: chrono::NaiveDate,
        min_age: u8,
        max_age: u8,
        excluded_profile_ids: Vec<Snowflake>,
    ) -> Result<Vec<UserProfile>, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let profiles = PgUserProfileRepository::recommend(
            &mut *conn,
            user_id,
            location,
            radius_km,
            gender,
            orientation,
            birth_date,
            min_age,
            max_age,
            excluded_profile_ids,
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

        if profile.avatar_hash.is_none() {
            return Err(UserProfileError::AvatarNotSet);
        }

        let liked_profile = PgUserProfileRepository::get_by_id(&mut *tx, liked_profile_id).await?;

        PgUserProfileRepository::add_like(&mut *tx, profile.id, liked_profile_id).await?;

        let fame_multiplier = FameCalculator::calculate_fame_multiplier(profile.rating);
        let fame_increase = FameCalculator::calculate_fame(liked_profile.rating, fame_multiplier);

        PgUserProfileRepository::increase_fame_rating(&mut *tx, liked_profile_id, fame_increase).await?;

        // TODO: this is a temporary solution
        let is_matched = PgUserProfileRepository::is_like_exists(&mut *tx, liked_profile_id, profile.id).await?;

        if is_matched {
            let channel_name = format!("dm-{}-{}", liked_profile_id, profile.id);

            let channel = PgChannelRepository::insert(&mut *tx, &ChannelInsert { name: channel_name }).await?;

            PgChannelRepository::add_participants(&mut *tx, channel.id, vec![liked_profile_id, profile.id]).await?;
        }

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

        if let Ok(channel) = PgChannelRepository::get_dm_channel(&mut *tx, profile_id, liked_profile_id).await {
            PgChannelRepository::delete(&mut *tx, channel.id).await?;
        }

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
    async fn add_dislike(&self, profile_id: Snowflake, disliked_profile_id: Snowflake) -> Result<(), UserProfileError> {
        let mut conn = self.redis.get_multiplexed_async_connection().await?;

        let key = format!("disliked_profile:{}", profile_id);
        conn.set_ex(&key, disliked_profile_id, DISLIKED_PROFILE_TTL).await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn get_disliked_ids(&self, profile_id: Snowflake) -> Result<Vec<Snowflake>, UserProfileError> {
        let mut conn = self.redis.get_multiplexed_async_connection().await?;

        let key = format!("disliked_profile:{}", profile_id);
        let disliked_profile_ids: Vec<Snowflake> = conn.get(&key).await?;

        Ok(disliked_profile_ids)
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

        let result = PgUserProfileRepository::view_profile(&mut *tx, profile_id, viewed_profile_id).await;

        if let Err(e) = result {
            if let sqlx::Error::Database(ref db_err) = e {
                if db_err.constraint() == Some("profile_view_user_profile_id_viewer_profile_id_key") {
                    return Ok(());
                }
            }
            return Err(e.into());
        }

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn get_viewers(&self, profile_id: Snowflake) -> Result<Vec<UserProfile>, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let profiles = PgUserProfileRepository::get_viewers(&mut *conn, profile_id).await?;

        Ok(profiles)
    }

    #[tracing::instrument(skip(self))]
    async fn block_user(&self, profile_id: Snowflake, blocked_profile_id: Snowflake) -> Result<(), UserProfileError> {
        let mut tx = self.pool.begin().await?;

        PgUserProfileRepository::block_user(&mut *tx, profile_id, blocked_profile_id).await?;

        if let Ok(channel) = PgChannelRepository::get_dm_channel(&mut *tx, profile_id, blocked_profile_id).await {
            PgChannelRepository::delete(&mut *tx, channel.id).await?;
        }

        PgUserProfileRepository::remove_like(&mut *tx, profile_id, blocked_profile_id).await?;
        PgUserProfileRepository::remove_like(&mut *tx, blocked_profile_id, profile_id).await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn unblock_user(&self, profile_id: Snowflake, blocked_profile_id: Snowflake) -> Result<(), UserProfileError> {
        let mut tx = self.pool.begin().await?;

        PgUserProfileRepository::unblock_user(&mut *tx, profile_id, blocked_profile_id).await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn is_blocked(&self, profile_id: Snowflake, blocked_profile_id: Snowflake) -> Result<bool, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let is_blocked = PgUserProfileRepository::is_blocked(&mut *conn, profile_id, blocked_profile_id).await?;

        Ok(is_blocked)
    }

    #[tracing::instrument(skip(self))]
    async fn get_blocked_users(&self, profile_id: Snowflake) -> Result<Vec<UserProfile>, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let profiles = PgUserProfileRepository::get_blocked_users(&mut *conn, profile_id).await?;

        Ok(profiles)
    }

    #[tracing::instrument(skip(self))]
    async fn get_blocked_user_ids(&self, profile_id: Snowflake) -> Result<Vec<Snowflake>, UserProfileError> {
        let mut conn = self.pool.acquire().await?;

        let profile_ids = PgUserProfileRepository::get_blocked_user_ids(&mut *conn, profile_id).await?;

        Ok(profile_ids)
    }

    #[tracing::instrument(skip(self))]
    async fn report_profile(
        &self,
        profile_id: Snowflake,
        reported_profile_id: Snowflake,
        reason: Option<&str>,
        block: bool,
    ) -> Result<(), UserProfileError> {
        let mut tx = self.pool.begin().await?;

        PgUserProfileRepository::report_profile(&mut *tx, profile_id, reported_profile_id, reason).await?;

        if block {
            Self::block_user(self, profile_id, reported_profile_id).await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
