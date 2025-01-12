use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::Acquire;

use crate::domain::entities::user_profile::UserProfile;
use crate::domain::repositories::repository::{QueryParams, DEFAULT_LIMIT, DEFAULT_OFFSET};
use crate::infrastructure::models::user_profile::{UserProfileInsert, UserProfileUpdate};
use crate::shared::types::snowflake::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl QueryParams for UserProfileQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait UserProfileRepository<Db>: Send + Sync {
    async fn insert<'a, A>(conn: A, profile: &UserProfileInsert) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_by_id<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<UserProfile, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_by_user_id<'a, A>(conn: A, user_id: Snowflake) -> sqlx::Result<UserProfile, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn update<'a, A>(
        conn: A,
        id: Snowflake,
        profile: UserProfileUpdate,
    ) -> sqlx::Result<UserProfile, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;
}
