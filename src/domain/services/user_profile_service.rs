use async_trait::async_trait;

use crate::domain::entities::profile_tag::ProfileTag;
use crate::domain::entities::user_profile::UserProfile;
use crate::domain::errors::user_profile_error::UserProfileError;
use crate::domain::repositories::user_profile_repo::UserProfileQueryParams;
use crate::infrastructure::models::user_profile::{UserProfileInsert, UserProfileUpdate};
use crate::shared::types::snowflake::Snowflake;

#[async_trait]
pub trait UserProfileService: 'static + Sync + Send {
    async fn create(&self, profile: &UserProfileInsert) -> Result<(), UserProfileError>;
    async fn get_by_id(&self, profile_id: Snowflake) -> Result<UserProfile, UserProfileError>;
    async fn get_by_user_id(&self, user_id: Snowflake) -> Result<UserProfile, UserProfileError>;
    async fn update(&self, id: Snowflake, profile: &UserProfileUpdate) -> Result<(), UserProfileError>;
    async fn search(&self, params: UserProfileQueryParams) -> Result<Vec<UserProfile>, UserProfileError>;
    async fn get_profile_tags(&self, profile_id: Snowflake) -> Result<Vec<ProfileTag>, UserProfileError>;
    async fn add_tag(&self, profile_id: Snowflake, tag_id: Snowflake) -> Result<(), UserProfileError>;
    async fn remove_tag(&self, profile_id: Snowflake, tag_id: Snowflake) -> Result<(), UserProfileError>;
    async fn bulk_add_tags(&self, profile_id: Snowflake, tag_ids: Vec<Snowflake>) -> Result<(), UserProfileError>;
    async fn bulk_remove_tags(&self, profile_id: Snowflake, tag_ids: Vec<Snowflake>) -> Result<(), UserProfileError>;
}
