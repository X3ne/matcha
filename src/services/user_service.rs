use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::user::User;
use crate::domain::errors::user_error::UserError;
use crate::domain::repositories::user_repo::UserRepository;
use crate::domain::services::user_service::UserService;
use crate::infrastructure::models::user::UserUpdate;
use crate::infrastructure::repositories::user_repo::PgUserRepository;
use crate::shared::types::snowflake::Snowflake;

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
    #[tracing::instrument(skip(self))]
    async fn get_by_id(&self, user_id: Snowflake) -> Result<User, UserError> {
        let mut conn = self.pool.acquire().await?;

        let user = PgUserRepository::get_by_id(&mut *conn, user_id).await?;

        Ok(user)
    }

    #[tracing::instrument(skip(self))]
    async fn update(&self, user_id: Snowflake, user: &UserUpdate) -> Result<(), UserError> {
        let mut tx = self.pool.begin().await?;

        PgUserRepository::update(&mut *tx, user_id, user).await?;

        tx.commit().await?;

        Ok(())
    }
}
