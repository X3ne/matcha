use async_trait::async_trait;
use sqlx::{Acquire, Postgres};

use crate::domain::entities::oauth_provider::OAuthProvider;
use crate::domain::repositories::oauth_provider_repo::OAuthProviderRepository;
use crate::infrastructure::models::oauth::{OAuthProviderInsert, OAuthProviderSqlx, OAuthProviderUpdate};
use crate::shared::types::snowflake::Snowflake;

pub struct PgOAuthProviderRepository;

#[async_trait]
impl OAuthProviderRepository<Postgres> for PgOAuthProviderRepository {
    #[tracing::instrument(skip(conn))]
    async fn insert<'a, A>(conn: A, provider: &OAuthProviderInsert) -> Result<OAuthProvider, sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let id = Snowflake::new();

        let provider = sqlx::query_as!(
            OAuthProviderSqlx,
            r#"
            INSERT INTO oauth_provider (id, name, active)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            id.as_i64(),
            provider.name,
            provider.active
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(OAuthProvider::from_db(&provider))
    }

    #[tracing::instrument(skip(conn))]
    async fn get_by_id<'a, A>(conn: A, id: Snowflake) -> Result<OAuthProvider, sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let provider = sqlx::query_as!(
            OAuthProviderSqlx,
            r#"
            SELECT *
            FROM oauth_provider
            WHERE id = $1
            "#,
            id.as_i64()
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(OAuthProvider::from_db(&provider))
    }

    #[tracing::instrument(skip(conn))]
    async fn get_by_name<'a, A>(conn: A, name: &str) -> Result<OAuthProvider, sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let provider = sqlx::query_as!(
            OAuthProviderSqlx,
            r#"
            SELECT *
            FROM oauth_provider
            WHERE name = $1
            "#,
            name
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(OAuthProvider::from_db(&provider))
    }

    #[tracing::instrument(skip(conn))]
    async fn get_all<'a, A>(conn: A) -> Result<Vec<OAuthProvider>, sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let providers = sqlx::query_as!(
            OAuthProviderSqlx,
            r#"
            SELECT *
            FROM oauth_provider
            "#,
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(providers
            .into_iter()
            .map(|provider: OAuthProviderSqlx| OAuthProvider::from_db(&provider))
            .collect())
    }

    #[tracing::instrument(skip(conn))]
    async fn update<'a, A>(
        conn: A,
        provider_id: Snowflake,
        provider: &OAuthProviderUpdate,
    ) -> Result<OAuthProvider, sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let updated_provider = sqlx::query_as!(
            OAuthProviderSqlx,
            r#"
            UPDATE oauth_provider
            SET name = $2, active = $3
            WHERE id = $1
            RETURNING *
            "#,
            provider_id.as_i64(),
            provider.name,
            provider.active
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(OAuthProvider::from_db(&updated_provider))
    }

    #[tracing::instrument(skip(conn))]
    async fn delete<'a, A>(conn: A, id: Snowflake) -> Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        sqlx::query_as!(
            OAuthProviderSqlx,
            r#"
            DELETE FROM oauth_provider
            WHERE id = $1
            "#,
            id.as_i64()
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(())
    }
}
