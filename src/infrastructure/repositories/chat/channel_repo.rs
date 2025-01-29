use async_trait::async_trait;
use sqlx::{Acquire, Error, Postgres};

use crate::domain::entities::chat::{Channel, ChannelParticipant};
use crate::domain::repositories::chat::channel_repository::{ChannelQueryParams, ChannelRepository};
use crate::domain::repositories::repository::QueryParams;
use crate::infrastructure::models::chat::{ChannelInsert, ChannelParticipantSqlx, ChannelSqlx, ChannelUpdate};
use crate::shared::types::snowflake::Snowflake;

pub struct PgChannelRepository;

#[async_trait]
impl ChannelRepository<Postgres> for PgChannelRepository {
    #[tracing::instrument(skip(conn))]
    async fn insert<'a, A>(conn: A, channel: &ChannelInsert) -> sqlx::Result<Channel, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let id = Snowflake::new();

        let channel = sqlx::query_as!(
            ChannelSqlx,
            r#"
            INSERT INTO channel (id, name)
            VALUES ($1, $2)
            RETURNING *
            "#,
            id.as_i64(),
            channel.name
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(channel.into())
    }

    async fn get_by_id<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<Channel, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let channel = sqlx::query_as!(
            ChannelSqlx,
            r#"
            SELECT *
            FROM channel
            WHERE id = $1
            "#,
            id.as_i64()
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(channel.into())
    }

    async fn get_dm_channel<'a, A>(
        conn: A,
        profile1_id: Snowflake,
        profile2_id: Snowflake,
    ) -> sqlx::Result<Channel, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let channel = sqlx::query_as!(
            ChannelSqlx,
            r#"
            SELECT c.*
            FROM channel c
            JOIN channel_participant cp1 ON c.id = cp1.channel_id
            JOIN channel_participant cp2 ON c.id = cp2.channel_id
            WHERE cp1.profile_id = $1 AND cp2.profile_id = $2
            "#,
            profile1_id.as_i64(),
            profile2_id.as_i64()
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(channel.into())
    }

    async fn get_profile_channels<'a, A>(
        conn: A,
        profile_id: Snowflake,
        params: &ChannelQueryParams,
    ) -> sqlx::Result<Vec<Channel>, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let query = format!(
            r#"
            SELECT c.*
            FROM channel c
            JOIN channel_participant cp ON c.id = cp.channel_id
            WHERE cp.profile_id = $1
            ORDER BY {sort_by} {sort_order}
            LIMIT $2
            OFFSET $3
            "#,
            sort_by = params.sort_by(),
            sort_order = params.sort_order()
        );

        let channels = sqlx::query_as::<_, ChannelSqlx>(&query)
            .bind(profile_id.as_i64())
            .bind(params.limit())
            .bind(params.offset())
            .fetch_all(&mut *conn)
            .await?;

        Ok(channels.into_iter().map(|c| c.into()).collect())
    }

    async fn get_participants<'a, A>(conn: A, channel_id: Snowflake) -> sqlx::Result<Vec<ChannelParticipant>, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let participants = sqlx::query_as!(
            ChannelParticipantSqlx,
            r#"
            SELECT cp.*, up.avatar_hash, up.name
            FROM channel_participant cp
            JOIN user_profile up ON cp.profile_id = up.id
            WHERE cp.channel_id = $1
            "#,
            channel_id.as_i64()
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(participants.into_iter().map(|cp| cp.into()).collect())
    }

    async fn add_participant<'a, A>(conn: A, channel_id: Snowflake, profile_id: Snowflake) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let id = Snowflake::new();

        sqlx::query!(
            r#"
            INSERT INTO channel_participant (id, channel_id, profile_id)
            VALUES ($1, $2, $3)
            "#,
            id.as_i64(),
            channel_id.as_i64(),
            profile_id.as_i64()
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    async fn add_participants<'a, A>(
        conn: A,
        channel_id: Snowflake,
        profile_ids: Vec<Snowflake>,
    ) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        for profile_id in profile_ids {
            let id = Snowflake::new();

            sqlx::query!(
                r#"
                INSERT INTO channel_participant (id, channel_id, profile_id)
                VALUES ($1, $2, $3)
                "#,
                id.as_i64(),
                channel_id.as_i64(),
                profile_id.as_i64()
            )
            .execute(&mut *conn)
            .await?;
        }

        Ok(())
    }

    async fn update<'a, A>(conn: A, id: Snowflake, channel: &ChannelUpdate) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        sqlx::query!(
            r#"
            UPDATE channel
            SET name = COALESCE($2, name)
            WHERE id = $1
            "#,
            id.as_i64(),
            channel.name
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    async fn delete<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        sqlx::query!(
            r#"
            DELETE FROM channel
            WHERE id = $1
            "#,
            id.as_i64()
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}
