use crate::domain::entities::user::User;
use crate::domain::repositories::repository::{QueryParams, DEFAULT_LIMIT, DEFAULT_OFFSET};
use crate::infrastructure::models::user::UserInsert;
use crate::shared::types::snowflake::Snowflake;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::Acquire;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl QueryParams for UserQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait UserRepository<Db>: Send + Sync {
    async fn insert<'a, A>(conn: A, user: &UserInsert) -> sqlx::Result<User, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_by_id<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<User, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_by_email<'a, A>(conn: A, email: &str) -> sqlx::Result<User, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn delete<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;
}
