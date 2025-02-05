use async_trait::async_trait;
use sqlx::{Acquire, Error, Postgres, Result};

use crate::domain::entities::user::User;
use crate::domain::repositories::user_repo::UserRepository;
use crate::infrastructure::models::user::{UserInsert, UserSqlx, UserUpdate};
use crate::shared::types::snowflake::Snowflake;
use crate::shared::utils::generate_random_secure_string;

pub struct PgUserRepository;

#[async_trait]
impl UserRepository<Postgres> for PgUserRepository {
    #[tracing::instrument(skip(conn))]
    async fn insert<'a, A>(conn: A, user: &UserInsert) -> Result<User, sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let id = Snowflake::new();

        let result = sqlx::query_as!(
            UserSqlx,
            r#"
            INSERT INTO "user" (id, email, username, last_name, first_name, password, activation_token)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            id.as_i64(),
            user.email,
            user.username,
            user.last_name,
            user.first_name,
            user.password,
            generate_random_secure_string(32)
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.into())
    }

    #[tracing::instrument(skip(conn))]
    async fn get_by_id<'a, A>(conn: A, id: Snowflake) -> Result<User, sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let user = sqlx::query_as!(
            UserSqlx,
            r#"
            SELECT *
            FROM "user"
            WHERE id = $1
            "#,
            id.as_i64()
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(user.into())
    }

    #[tracing::instrument(skip(conn))]
    async fn get_by_email<'a, A>(conn: A, email: &str) -> Result<User, sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let user = sqlx::query_as!(
            UserSqlx,
            r#"
            SELECT *
            FROM "user"
            WHERE email = $1
            "#,
            email
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(user.into())
    }

    #[tracing::instrument(skip(conn))]
    async fn get_by_username<'a, A>(conn: A, username: &str) -> Result<User, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let user = sqlx::query_as!(
            UserSqlx,
            r#"
            SELECT *
            FROM "user"
            WHERE username = $1
            "#,
            username
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(user.into())
    }

    #[tracing::instrument(skip(conn))]
    async fn update<'a, A>(conn: A, id: Snowflake, user: &UserUpdate) -> Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        sqlx::query!(
            r#"
            UPDATE "user"
            SET email = COALESCE($2, email),
                username = COALESCE($3, username),
                last_name = COALESCE($4, last_name),
                first_name = COALESCE($5, first_name)
            WHERE id = $1
            "#,
            id.as_i64(),
            user.email,
            user.username,
            user.last_name,
            user.first_name,
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(conn))]
    async fn update_password<'a, A>(conn: A, email: &str, password: &str) -> Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        sqlx::query!(
            r#"
            UPDATE "user"
            SET password = $2
            WHERE email = $1
            "#,
            email,
            password
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(conn))]
    async fn activate<'a, A>(conn: A, token: String) -> Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        sqlx::query!(
            r#"
            UPDATE "user"
            SET is_active = true
            WHERE activation_token = $1
            "#,
            token
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(conn))]
    async fn delete<'a, A>(conn: A, id: Snowflake) -> Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        sqlx::query!(
            r#"
            DELETE FROM "user"
            WHERE id = $1
            "#,
            id.as_i64()
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}
