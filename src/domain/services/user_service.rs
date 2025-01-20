use async_trait::async_trait;

use crate::domain::entities::user::User;
use crate::domain::errors::user_error::UserError;
use crate::infrastructure::models::user::UserUpdate;
use crate::shared::types::snowflake::Snowflake;

#[async_trait]
pub trait UserService: 'static + Sync + Send {
    async fn get_by_id(&self, user_id: Snowflake) -> Result<User, UserError>;
    async fn get_by_email(&self, email: &str) -> Result<User, UserError>;
    async fn update(&self, user_id: Snowflake, user: &UserUpdate) -> Result<(), UserError>;
}
