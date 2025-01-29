use async_trait::async_trait;
use sqlx::{Acquire, Error, Postgres};

use crate::domain::entities::chat::Message;
use crate::domain::repositories::chat::message_repository::{MessageQueryParams, MessageRepository};
use crate::domain::repositories::repository::QueryParams;
use crate::infrastructure::models::chat::{MessageInsert, MessageSqlx, MessageUpdate};
use crate::shared::types::snowflake::Snowflake;

pub struct PgMessageRepository;

#[async_trait]
impl MessageRepository<Postgres> for PgMessageRepository {
    #[tracing::instrument(skip(conn))]
    async fn insert<'a, A>(conn: A, message: &MessageInsert) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;
        let id = Snowflake::new();

        sqlx::query!(
            r#"
            INSERT INTO message (id, author_id, channel_id, content, sent_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            id.as_i64(),
            message.author_id.as_i64(),
            message.channel_id.as_i64(),
            message.content,
            message.sent_at
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(conn))]
    async fn get_by_id<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<Message, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let message = sqlx::query_as!(
            MessageSqlx,
            r#"
            SELECT 
                m.id,
                m.author_id,
                u.name AS author_name,
                u.avatar_hash AS author_avatar_hash,
                m.channel_id,
                m.content,
                m.deleted,
                m.sent_at,
                m.edited_at
            FROM message m
            JOIN user_profile u ON m.author_id = u.id
            WHERE m.id = $1
            "#,
            id.as_i64()
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(message.into())
    }

    #[tracing::instrument(skip(conn))]
    async fn get_channel_messages<'a, A>(
        conn: A,
        channel_id: Snowflake,
        params: &MessageQueryParams,
    ) -> sqlx::Result<Vec<Message>, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let query = format!(
            r#"
            SELECT 
                m.id,
                m.author_id,
                u.name AS author_name,
                u.avatar_hash AS author_avatar_hash,
                m.channel_id,
                m.content,
                m.deleted,
                m.sent_at,
                m.edited_at
            FROM message m
            JOIN user_profile u ON m.author_id = u.id
            WHERE m.channel_id = $1
            AND m.deleted = FALSE
            ORDER BY {sort_by} {sort_order}
            LIMIT $2 OFFSET $3
            "#,
            sort_by = params.sort_by(),
            sort_order = params.sort_order()
        );

        let messages = sqlx::query_as::<_, MessageSqlx>(&query)
            .bind(channel_id.as_i64())
            .bind(params.limit())
            .bind(params.offset())
            .fetch_all(&mut *conn)
            .await?;

        Ok(messages.into_iter().map(Message::from).collect())
    }

    #[tracing::instrument(skip(conn))]
    async fn update<'a, A>(conn: A, id: Snowflake, message: &MessageUpdate) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        sqlx::query!(
            r#"
            UPDATE message
            SET content = $1
            WHERE id = $2
            "#,
            message.content,
            id.as_i64()
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(conn))]
    async fn delete<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        sqlx::query!(
            r#"
            DELETE FROM message
            WHERE id = $1
            "#,
            id.as_i64()
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}
