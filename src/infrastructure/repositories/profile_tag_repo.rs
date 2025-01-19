use async_trait::async_trait;
use sqlx::{Acquire, Error, Postgres};

use crate::domain::entities::profile_tag::ProfileTag;
use crate::domain::repositories::profile_tag_repository::ProfileTagRepository;
use crate::infrastructure::models::profile_tag::{ProfileTagInsert, ProfileTagSqlx};
use crate::shared::types::snowflake::Snowflake;

pub struct PgProfileTagRepository;

#[async_trait]
impl ProfileTagRepository<Postgres> for PgProfileTagRepository {
    async fn insert<'a, A>(conn: A, tag: &ProfileTagInsert) -> sqlx::Result<ProfileTag, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let id = Snowflake::new();

        let result = sqlx::query_as!(
            ProfileTagSqlx,
            r#"
            INSERT INTO profile_tag (id, name)
            VALUES ($1, $2)
            RETURNING *
            "#,
            id.as_i64(),
            tag.name
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.into())
    }

    async fn get_by_id<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<ProfileTag, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let result = sqlx::query_as!(
            ProfileTagSqlx,
            r#"
            SELECT *
            FROM profile_tag
            WHERE id = $1
            "#,
            id.as_i64()
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.into())
    }

    async fn get_by_ids<'a, A>(conn: A, ids: Vec<Snowflake>) -> sqlx::Result<Vec<ProfileTag>, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let result = sqlx::query_as!(
            ProfileTagSqlx,
            r#"
            SELECT *
            FROM profile_tag
            WHERE id = ANY($1)
            "#,
            &ids.iter().map(|id| id.as_i64()).collect::<Vec<_>>()
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(result.into_iter().map(|tag| tag.into()).collect())
    }

    async fn get_by_name<'a, A>(conn: A, name: &str) -> sqlx::Result<ProfileTag, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let result = sqlx::query_as!(
            ProfileTagSqlx,
            r#"
            SELECT *
            FROM profile_tag
            WHERE name = $1
            "#,
            name
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.into())
    }

    async fn get_all<'a, A>(conn: A) -> sqlx::Result<Vec<ProfileTag>, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let result = sqlx::query_as!(
            ProfileTagSqlx,
            r#"
            SELECT *
            FROM profile_tag
            "#,
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(result.into_iter().map(|tag| tag.into()).collect())
    }

    async fn delete<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        sqlx::query!(
            r#"
            DELETE FROM profile_tag
            WHERE id = $1
            "#,
            id.as_i64()
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}
