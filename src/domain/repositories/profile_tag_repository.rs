use async_trait::async_trait;
use sqlx::Acquire;

use crate::domain::entities::profile_tag::ProfileTag;
use crate::infrastructure::models::profile_tag::ProfileTagInsert;
use crate::shared::types::snowflake::Snowflake;

#[async_trait]
#[allow(dead_code)]
pub trait ProfileTagRepository<Db>: Send + Sync {
    async fn insert<'a, A>(conn: A, tag: &ProfileTagInsert) -> sqlx::Result<ProfileTag, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_by_id<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<ProfileTag, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_by_ids<'a, A>(conn: A, ids: Vec<Snowflake>) -> sqlx::Result<Vec<ProfileTag>, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_by_name<'a, A>(conn: A, name: &str) -> sqlx::Result<ProfileTag, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_all<'a, A>(conn: A) -> sqlx::Result<Vec<ProfileTag>, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn delete<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;
}
