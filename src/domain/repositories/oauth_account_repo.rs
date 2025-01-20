use async_trait::async_trait;
use oauth2::client::providers::ProviderKind;
use sqlx::Acquire;

use crate::domain::entities::oauth_account::OAuthAccount;
use crate::infrastructure::models::oauth::{OAuthAccountInsert, OAuthAccountUpdate};
use crate::shared::types::snowflake::Snowflake;

#[async_trait]
#[allow(dead_code)]
pub trait OAuthAccountRepository<Db>: Send + Sync {
    async fn insert<'a, A>(conn: A, account: &OAuthAccountInsert) -> sqlx::Result<OAuthAccount, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_by_id<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<OAuthAccount, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_by_user_id<'a, A>(conn: A, user_id: Snowflake) -> sqlx::Result<Vec<OAuthAccount>, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_by_provider_user_and_provider<'a, A>(
        conn: A,
        provider_user_id: &str,
        provider: ProviderKind,
    ) -> sqlx::Result<OAuthAccount, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn update<'a, A>(
        conn: A,
        id: Snowflake,
        user: &OAuthAccountUpdate,
    ) -> sqlx::Result<OAuthAccount, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn delete<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;
}
