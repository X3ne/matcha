use async_trait::async_trait;
use oauth2::client::providers::ProviderKind;
use sqlx::{Acquire, Postgres};

use crate::domain::entities::oauth_account::OAuthAccount;
use crate::domain::repositories::oauth_account_repo::OAuthAccountRepository;
use crate::infrastructure::models::oauth::{OAuthAccountInsert, OAuthAccountSqlx, OAuthAccountUpdate};
use crate::shared::types::snowflake::{Snowflake, SNOWFLAKE_GENERATOR};

pub struct PgOAuthAccountRepository;

#[async_trait]
impl OAuthAccountRepository<Postgres> for PgOAuthAccountRepository {
    async fn insert<'a, A>(conn: A, account: &OAuthAccountInsert) -> Result<OAuthAccount, sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let id = SNOWFLAKE_GENERATOR.generate();

        let account = sqlx::query_as!(
            OAuthAccountSqlx,
            r#"
            INSERT INTO oauth_account (id, user_id, provider_id, provider_user_id, access_token, refresh_token, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            id.as_i64(),
            account.user_id.as_i64(),
            account.provider_id.as_i64(),
            account.provider_user_id,
            account.access_token,
            account.refresh_token,
            account.expires_at
        )
            .fetch_one(&mut *conn)
            .await?;

        Ok(OAuthAccount::from_db(&account))
    }

    async fn get_by_id<'a, A>(conn: A, id: Snowflake) -> Result<OAuthAccount, sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let account = sqlx::query_as!(
            OAuthAccountSqlx,
            r#"
            SELECT *
            FROM oauth_account
            WHERE id = $1
            "#,
            id.as_i64(),
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(OAuthAccount::from_db(&account))
    }

    async fn get_by_user_id<'a, A>(conn: A, user_id: Snowflake) -> Result<Vec<OAuthAccount>, sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let accounts = sqlx::query_as!(
            OAuthAccountSqlx,
            r#"
            SELECT *
            FROM oauth_account
            WHERE user_id = $1
            "#,
            user_id.as_i64()
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(accounts
            .into_iter()
            .map(|account: OAuthAccountSqlx| OAuthAccount::from_db(&account))
            .collect())
    }

    async fn get_by_provider_user_and_provider<'a, A>(
        conn: A,
        provider_user_id: &str,
        provider: ProviderKind,
    ) -> Result<OAuthAccount, sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let account = sqlx::query_as!(
            OAuthAccountSqlx,
            r#"
            SELECT oauth_account.*
            FROM oauth_account
            JOIN oauth_provider ON oauth_account.provider_id = oauth_provider.id
            WHERE oauth_account.provider_user_id = $1 AND oauth_provider.name = $2
            "#,
            provider_user_id,
            provider.to_string()
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(OAuthAccount::from_db(&account))
    }

    async fn update<'a, A>(
        conn: A,
        account_id: Snowflake,
        account: &OAuthAccountUpdate,
    ) -> Result<OAuthAccount, sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let updated_account = sqlx::query_as!(
            OAuthAccountSqlx,
            r#"
            UPDATE oauth_account
            SET access_token = $2, refresh_token = $3, expires_at = $4
            WHERE id = $1
            RETURNING *
            "#,
            account_id.as_i64(),
            account.access_token,
            account.refresh_token,
            account.expires_at
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(OAuthAccount::from_db(&updated_account))
    }

    async fn delete<'a, A>(conn: A, id: Snowflake) -> Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let deleted_account = sqlx::query_as!(
            OAuthAccountDb,
            r#"
            DELETE FROM oauth_account
            WHERE id = $1
            "#,
            id.as_i64()
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(())
    }
}
