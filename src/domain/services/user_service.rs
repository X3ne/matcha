use crate::domain::entities::user::User;
use crate::domain::errors::user_error::UserError;
use crate::shared::types::snowflake::Snowflake;
use async_trait::async_trait;

#[async_trait]
pub trait UserService: 'static + Sync + Send {
    async fn get_by_id(&self, user_id: Snowflake) -> Result<User, UserError>;
}
