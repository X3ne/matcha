use async_trait::async_trait;
use sqlx::Acquire;

use crate::domain::entities::oauth_provider::OAuthProvider;
use crate::infrastructure::models::oauth::{OAuthProviderInsert, OAuthProviderUpdate};
use crate::shared::types::snowflake::Snowflake;

#[async_trait]
pub trait OAuthProviderRepository<Db> {
    async fn insert<'a, A>(conn: A, provider: &OAuthProviderInsert) -> sqlx::Result<OAuthProvider, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_by_id<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<OAuthProvider, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_by_name<'a, A>(conn: A, name: &str) -> sqlx::Result<OAuthProvider, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_all<'a, A>(conn: A) -> sqlx::Result<Vec<OAuthProvider>, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn update<'a, A>(
        conn: A,
        id: Snowflake,
        user: &OAuthProviderUpdate,
    ) -> sqlx::Result<OAuthProvider, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn delete<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;
}
