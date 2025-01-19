use crate::domain::entities::profile_tag::ProfileTag;
use crate::domain::errors::profile_tag_error::ProfileTagError;
use crate::shared::types::snowflake::Snowflake;
use async_trait::async_trait;

#[async_trait]
pub trait ProfileTagService: 'static + Sync + Send {
    async fn get_by_id(&self, tag_id: Snowflake) -> Result<ProfileTag, ProfileTagError>;
    async fn get_by_ids(&self, tag_ids: Vec<Snowflake>) -> Result<Vec<ProfileTag>, ProfileTagError>;
    async fn get_all(&self) -> Result<Vec<ProfileTag>, ProfileTagError>;
}
