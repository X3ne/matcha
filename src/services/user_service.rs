use std::sync::Arc;

use crate::domain::entities::user::User;
use crate::domain::errors::user_error::UserError;
use crate::domain::repositories::user_repo::UserRepository;
use crate::domain::services::user_service::UserService;
use crate::infrastructure::repositories::user_repo::PgUserRepository;
use crate::shared::types::snowflake::Snowflake;
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Clone)]
pub struct UserServiceImpl {
    pub pool: Arc<PgPool>,
}

impl UserServiceImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        UserServiceImpl { pool }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_by_id(&self, user_id: Snowflake) -> Result<User, UserError> {
        let mut conn = self.pool.acquire().await?;

        let user = PgUserRepository::get_by_id(&mut *conn, user_id).await?;

        Ok(user)
    }
}
