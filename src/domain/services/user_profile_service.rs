use async_trait::async_trait;

use crate::domain::entities::profile_tag::ProfileTag;
use crate::domain::entities::user_profile::UserProfile;
use crate::domain::errors::user_profile_error::UserProfileError;
use crate::domain::repositories::user_profile_repo::UserProfileQueryParams;
use crate::infrastructure::models::user_profile::{UserProfileInsert, UserProfileUpdate};
use crate::shared::types::snowflake::Snowflake;
use crate::shared::types::user_profile::{Gender, Orientation};

#[async_trait]
pub trait UserProfileService: 'static + Sync + Send {
    async fn create(&self, profile: &UserProfileInsert) -> Result<(), UserProfileError>;
    async fn get_by_id(&self, profile_id: Snowflake) -> Result<UserProfile, UserProfileError>;
    async fn get_by_user_id(&self, user_id: Snowflake) -> Result<UserProfile, UserProfileError>;
    async fn update(&self, id: Snowflake, profile: &UserProfileUpdate) -> Result<(), UserProfileError>;
    async fn search(
        &self,
        params: &UserProfileQueryParams,
        current_profile_id: Snowflake,
    ) -> Result<Vec<UserProfile>, UserProfileError>;
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
    ) -> Result<Vec<UserProfile>, UserProfileError>;
    async fn get_profile_tags(&self, profile_id: Snowflake) -> Result<Vec<ProfileTag>, UserProfileError>;
    async fn add_pictures(&self, profile_id: Snowflake, picture_hashes: Vec<String>) -> Result<(), UserProfileError>;
    async fn remove_pictures(&self, profile_id: Snowflake, picture_hashes: Vec<String>)
        -> Result<(), UserProfileError>;
    async fn add_tag(&self, profile_id: Snowflake, tag_id: Snowflake) -> Result<(), UserProfileError>;
    async fn remove_tag(&self, profile_id: Snowflake, tag_id: Snowflake) -> Result<(), UserProfileError>;
    async fn bulk_add_tags(&self, profile_id: Snowflake, tag_ids: Vec<Snowflake>) -> Result<(), UserProfileError>;
    async fn bulk_remove_tags(&self, profile_id: Snowflake, tag_ids: Vec<Snowflake>) -> Result<(), UserProfileError>;
    async fn add_like(&self, profile: &UserProfile, liked_profile_id: Snowflake) -> Result<(), UserProfileError>;
    async fn remove_like(&self, profile_id: Snowflake, liked_profile_id: Snowflake) -> Result<(), UserProfileError>;
    async fn is_profile_liked(
        &self,
        profile_id: Snowflake,
        liked_profile_id: Snowflake,
    ) -> Result<bool, UserProfileError>;
    async fn is_profile_matched(
        &self,
        profile_id: Snowflake,
        matched_profile_id: Snowflake,
    ) -> Result<bool, UserProfileError>;
    async fn get_my_likes(&self, profile_id: Snowflake) -> Result<Vec<UserProfile>, UserProfileError>;
    async fn get_profile_likes(&self, profile_id: Snowflake) -> Result<Vec<UserProfile>, UserProfileError>;
    async fn get_matches(&self, profile_id: Snowflake) -> Result<Vec<UserProfile>, UserProfileError>;
    async fn view_profile(&self, profile_id: Snowflake, viewed_profile_id: Snowflake) -> Result<(), UserProfileError>;
    async fn get_viewers(&self, profile_id: Snowflake) -> Result<Vec<UserProfile>, UserProfileError>;
}
