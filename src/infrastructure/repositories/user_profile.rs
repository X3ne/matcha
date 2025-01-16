use async_trait::async_trait;
use geozero::wkb;
use sqlx::{Acquire, Error, Postgres, QueryBuilder};

use crate::domain::entities::user_profile::UserProfile;
use crate::domain::repositories::repository::QueryParams;
use crate::domain::repositories::user_profile_repo::{UserProfileQueryParams, UserProfileRepository};
use crate::infrastructure::models::user_profile::{
    RawProfileWithTag, UserProfileInsert, UserProfileSqlx, UserProfileUpdate,
};
use crate::shared::types::snowflake::Snowflake;

pub struct PgUserProfileRepository;

#[async_trait]
impl UserProfileRepository<Postgres> for PgUserProfileRepository {
    async fn insert<'a, A>(conn: A, profile: &UserProfileInsert) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let id = Snowflake::new();

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

        for tag_id in &profile.tag_ids {
            let join_id = Snowflake::new();

            sqlx::query!(
                r#"
                INSERT INTO join_user_profile_tag (id, user_profile_id, profile_tag_id)
                VALUES ($1, $2, $3)
                "#,
                join_id.as_i64(),
                id.as_i64(),
                tag_id.as_i64()
            )
            .execute(&mut *conn)
            .await?;
        }

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
                up.rating,
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
                up.rating,
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

    async fn search<'a, A>(conn: A, params: UserProfileQueryParams) -> sqlx::Result<Vec<UserProfile>, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT DISTINCT up.* FROM user_profile up
            LEFT JOIN join_user_profile_tag jpt ON up.id = jpt.user_profile_id
            LEFT JOIN profile_tag pt ON jpt.profile_tag_id = pt.id
            WHERE 1=1",
        );

        if let Some(min_age) = params.min_age {
            query_builder.push(" AND up.age >= ");
            query_builder.push_bind(min_age);
        }
        if let Some(max_age) = params.max_age {
            query_builder.push(" AND up.age <= ");
            query_builder.push_bind(max_age);
        }
        if let Some(min_fame) = params.min_fame_rating {
            query_builder.push(" AND up.fame_rating >= ");
            query_builder.push_bind(min_fame);
        }
        if let Some(max_fame) = params.max_fame_rating {
            query_builder.push(" AND up.fame_rating <= ");
            query_builder.push_bind(max_fame);
        }

        if let Some(location) = params.location {
            if let Some(radius) = params.radius_km {
                let geom: geo_types::Geometry<f64> = location.into();
                query_builder.push(" AND ST_DWithin(up.location::geography, ST_SetSRID(ST_GeomFromEWKB(");
                query_builder.push_bind(wkb::Encode(geom));
                query_builder.push("), 4326)::geography, ");
                query_builder.push_bind(radius * 1000.0);
                query_builder.push(")");
            }
        }

        if let Some(tags) = &params.tag_ids {
            query_builder.push(" AND pt.id = ANY(");
            query_builder.push_bind(tags.iter().map(|tag_id| tag_id.to_string()).collect::<Vec<_>>());
            query_builder.push(")");
        }

        if let Some(sort_by) = &params.sort_by {
            query_builder.push(&format!(" ORDER BY up.{}", sort_by.to_string()));
        }

        if let Some(sort_order) = &params.sort_order {
            if params.sort_by.is_none() {
                query_builder.push(" ORDER BY up.id");
            }
            query_builder.push(&format!(" {}", sort_order.to_string()));
        }

        query_builder.push(" LIMIT ");
        query_builder.push_bind(params.limit());
        query_builder.push(" OFFSET ");
        query_builder.push_bind(params.offset());

        tracing::debug!("Generated SQL Query: {}", query_builder.sql());

        let profiles = query_builder
            .build_query_as::<UserProfileSqlx>()
            .fetch_all(&mut *conn)
            .await?;

        let profiles: Vec<UserProfile> = profiles.into_iter().map(|profile| profile.into()).collect();
        Ok(profiles)
    }
}
