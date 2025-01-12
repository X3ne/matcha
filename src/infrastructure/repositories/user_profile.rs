use async_trait::async_trait;
use geozero::wkb;
use sqlx::{Acquire, Error, Postgres};

use crate::domain::entities::user_profile::UserProfile;
use crate::domain::repositories::user_profile_repo::UserProfileRepository;
use crate::infrastructure::models::user_profile::{RawProfileWithTag, UserProfileInsert, UserProfileUpdate};
use crate::shared::types::snowflake::{Snowflake, SNOWFLAKE_GENERATOR};

pub struct PgUserProfileRepository;

#[async_trait]
impl UserProfileRepository<Postgres> for PgUserProfileRepository {
    async fn insert<'a, A>(conn: A, profile: &UserProfileInsert) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let id = SNOWFLAKE_GENERATOR.generate();

        let geom: geo_types::Geometry<f64> = profile.location.into();

        sqlx::query!(
            r#"
            INSERT INTO user_profile (id, user_id, name, avatar_hash, bio, age, gender, sexual_orientation, location)
            VALUES ($1, $2, $3, $4, $5, $6, $7::gender, $8::sexual_orientation, $9::geometry)
            "#,
            id.as_i64(),
            profile.user_id.as_i64(),
            profile.name,
            profile.avatar_hash,
            profile.bio,
            profile.age,
            profile.gender as _,
            profile.sexual_orientation as _,
            wkb::Encode(geom) as _
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    async fn get_by_id<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<UserProfile, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let rows: Vec<RawProfileWithTag> = sqlx::query_as!(
            RawProfileWithTag,
            r#"
            SELECT 
                up.id AS profile_id,
                up.user_id,
                up.name,
                up.avatar_hash,
                up.bio,
                up.age,
                up.gender AS "gender: _",
                up.sexual_orientation AS "sexual_orientation: _",
                up.location AS "location!: _",
                up.created_at,
                up.updated_at,
                pt.id AS "tag_id?: _",
                pt.name AS "tag_name?: _"
            FROM
                user_profile up
            LEFT JOIN
                join_user_profile_tag upt ON up.id = upt.user_profile_id
            LEFT JOIN
                profile_tag pt ON upt.profile_tag_id = pt.id
            WHERE
                up.id = $1
            "#,
            id.as_i64()
        )
        .fetch_all(&mut *conn)
        .await?;

        if rows.is_empty() {
            return Err(Error::RowNotFound);
        }

        let user_profile: UserProfile = rows.try_into()?;
        Ok(user_profile)
    }

    async fn get_by_user_id<'a, A>(conn: A, user_id: Snowflake) -> sqlx::Result<UserProfile, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let rows: Vec<RawProfileWithTag> = sqlx::query_as!(
            RawProfileWithTag,
            r#"
            SELECT 
                up.id AS profile_id,
                up.user_id,
                up.name,
                up.avatar_hash,
                up.bio,
                up.age,
                up.gender AS "gender: _",
                up.sexual_orientation AS "sexual_orientation: _",
                up.location AS "location!: _",
                up.created_at,
                up.updated_at,
                pt.id AS "tag_id?: _",
                pt.name AS "tag_name?: _"
            FROM
                user_profile up
            LEFT JOIN
                join_user_profile_tag upt ON up.id = upt.user_profile_id
            LEFT JOIN
                profile_tag pt ON upt.profile_tag_id = pt.id
            WHERE
                up.user_id = $1
            "#,
            user_id.as_i64()
        )
        .fetch_all(&mut *conn)
        .await?;

        if rows.is_empty() {
            return Err(Error::RowNotFound);
        }

        let user_profile: UserProfile = rows.try_into()?;
        Ok(user_profile)
    }

    async fn update<'a, A>(conn: A, id: Snowflake, profile: UserProfileUpdate) -> sqlx::Result<UserProfile, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        todo!()
    }
}
