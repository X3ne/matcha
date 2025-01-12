use async_trait::async_trait;

use crate::domain::entities::user_profile::UserProfile;
use crate::domain::errors::user_error::UserError;
use crate::infrastructure::models::user_profile::UserProfileInsert;
use crate::shared::types::snowflake::Snowflake;

#[async_trait]
pub trait UserProfileService: 'static + Sync + Send {
    async fn create(&self, profile: &UserProfileInsert) -> Result<(), UserError>;
    async fn get_by_id(&self, profile_id: Snowflake) -> Result<UserProfile, UserError>;
    async fn get_by_user_id(&self, user_id: Snowflake) -> Result<UserProfile, UserError>;
}
