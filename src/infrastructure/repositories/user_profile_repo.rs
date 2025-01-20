use async_trait::async_trait;
use geozero::wkb;
use sqlx::{Acquire, Error, Postgres, QueryBuilder};

use crate::domain::entities::profile_tag::ProfileTag;
use crate::domain::entities::user_profile::UserProfile;
use crate::domain::repositories::user_profile_repo::{
    UserProfileQueryParams, UserProfileRepository, UserProfileSortBy,
};
use crate::infrastructure::models::profile_tag::ProfileTagSqlx;
use crate::infrastructure::models::user_profile::{UserProfileInsert, UserProfileSqlx, UserProfileUpdate};
use crate::shared::types::snowflake::Snowflake;

pub struct PgUserProfileRepository;

#[async_trait]
impl UserProfileRepository<Postgres> for PgUserProfileRepository {
    #[tracing::instrument(skip(conn))]
    async fn insert<'a, A>(conn: A, profile: &UserProfileInsert) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let id = Snowflake::new();

        let geom: geo_types::Geometry<f64> = profile.location.into();

        sqlx::query!(
            r#"
            INSERT INTO user_profile (id, user_id, name, avatar_hash, picture_hashes, bio, age, gender, sexual_orientation, location)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8::gender, $9::sexual_orientation, $10::geometry)
            "#,
            id.as_i64(),
            profile.user_id.as_i64(),
            profile.name,
            profile.avatar_hash,
            &profile.picture_hashes,
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

    #[tracing::instrument(skip(conn))]
    async fn get_by_id<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<UserProfile, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let profile = sqlx::query_as!(
            UserProfileSqlx,
            r#"
            SELECT 
                id,
                user_id,
                name,
                avatar_hash,
                picture_hashes,
                bio,
                age,
                gender AS "gender: _",
                sexual_orientation AS "sexual_orientation: _",
                location AS "location!: _",
                rating,
                created_at,
                updated_at
            FROM
                user_profile
            WHERE
                id = $1
            "#,
            id.as_i64()
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(profile.into())
    }

    #[tracing::instrument(skip(conn))]
    async fn get_by_user_id<'a, A>(conn: A, user_id: Snowflake) -> sqlx::Result<UserProfile, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let profile = sqlx::query_as!(
            UserProfileSqlx,
            r#"
            SELECT 
                id,
                user_id,
                name,
                avatar_hash,
                picture_hashes,
                bio,
                age,
                gender AS "gender: _",
                sexual_orientation AS "sexual_orientation: _",
                location AS "location!: _",
                rating,
                created_at,
                updated_at
            FROM
                user_profile
            WHERE
                user_id = $1
            "#,
            user_id.as_i64()
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(profile.into())
    }

    #[tracing::instrument(skip(conn))]
    async fn update<'a, A>(conn: A, id: Snowflake, profile: &UserProfileUpdate) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        tracing::info!("Updating profile: {:?}", profile);

        let geom: Option<geo_types::Geometry<f64>> = profile.location.map(Into::into);
        let encode = geom.map(wkb::Encode);

        sqlx::query!(
            r#"
            UPDATE user_profile
            SET
                name = COALESCE($2, name),
                avatar_hash = COALESCE($3, avatar_hash),
                bio = COALESCE($4, bio),
                age = COALESCE($5, age),
                gender = COALESCE($6, gender),
                sexual_orientation = COALESCE($7, sexual_orientation),
                location = COALESCE($8, location)
            WHERE
                id = $1
            "#,
            id.as_i64(),
            profile.name,
            profile.avatar_hash,
            profile.bio,
            profile.age,
            profile.gender as _,
            profile.sexual_orientation as _,
            encode as _
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(conn))]
    async fn search<'a, A>(conn: A, params: &UserProfileQueryParams) -> sqlx::Result<Vec<UserProfile>, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("SELECT up.*, ");

        // calculate distance between profile and provided location
        if let Some(location) = params.location {
            let location: geo_types::Geometry<f64> = location.into();
            query_builder.push("ST_Distance(up.location::geography, ST_SetSRID(ST_GeomFromEWKB(");
            query_builder.push_bind(wkb::Encode(location));
            query_builder.push("), 4326)::geography) AS distance, ");
        } else {
            query_builder.push("NULL AS distance, ");
        }

        // count common tags
        if params.tag_ids.is_some() {
            query_builder.push("COUNT(DISTINCT pt.id) AS common_tags_count ");
        } else {
            query_builder.push("0 AS common_tags_count ");
        }

        query_builder.push(
            " FROM user_profile up
        LEFT JOIN join_user_profile_tag jpt ON up.id = jpt.user_profile_id
        LEFT JOIN profile_tag pt ON jpt.profile_tag_id = pt.id
        WHERE 1=1",
        );

        // filters
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

        // filtering within radius
        if let Some(location) = params.location {
            if let Some(radius) = params.radius_km {
                let location: geo_types::Geometry<f64> = location.into();
                query_builder.push(" AND ST_DWithin(up.location::geography, ST_SetSRID(ST_GeomFromEWKB(");
                query_builder.push_bind(wkb::Encode(location));
                query_builder.push("), 4326)::geography, ");
                query_builder.push_bind(radius * 1000.0);
                query_builder.push(")");
            }
        }

        // filtering by tags
        if let Some(tags) = &params.tag_ids {
            query_builder.push(" AND pt.id = ANY(");
            query_builder.push_bind(tags.iter().map(|tag_id| tag_id.as_i64()).collect::<Vec<_>>());
            query_builder.push(")");
        }

        query_builder.push(" GROUP BY up.id");

        match params.sort_by {
            Some(UserProfileSortBy::Distance) => {
                query_builder.push(" ORDER BY distance");
                if let Some(sort_order) = &params.sort_order {
                    query_builder.push(&format!(" {}", sort_order.to_string()));
                } else {
                    query_builder.push(" ASC");
                }
            }
            Some(UserProfileSortBy::Age) => {
                query_builder.push(" ORDER BY up.age");
                if let Some(sort_order) = &params.sort_order {
                    query_builder.push(&format!(" {}", sort_order.to_string()));
                } else {
                    query_builder.push(" ASC");
                }
            }
            Some(UserProfileSortBy::FameRating) => {
                query_builder.push(" ORDER BY up.fame_rating");
                if let Some(sort_order) = &params.sort_order {
                    query_builder.push(&format!(" {}", sort_order.to_string()));
                } else {
                    query_builder.push(" DESC");
                }
            }
            Some(UserProfileSortBy::Tags) => {
                query_builder.push(" ORDER BY common_tags_count");
                if let Some(sort_order) = &params.sort_order {
                    query_builder.push(&format!(" {}", sort_order.to_string()));
                } else {
                    query_builder.push(" DESC");
                }
            }
            _ => {}
        }

        // pagination
        query_builder.push(" LIMIT ");
        query_builder.push_bind(params.limit.unwrap_or(50));
        query_builder.push(" OFFSET ");
        query_builder.push_bind(params.offset.unwrap_or(0));

        tracing::debug!("Generated SQL Query: {}", query_builder.sql());

        let profiles = query_builder
            .build_query_as::<UserProfileSqlx>()
            .fetch_all(&mut *conn)
            .await?;

        let profiles: Vec<UserProfile> = profiles.into_iter().map(|profile| profile.into()).collect();
        Ok(profiles)
    }

    #[tracing::instrument(skip(conn))]
    async fn get_profile_tags<'a, A>(conn: A, profile_id: Snowflake) -> sqlx::Result<Vec<ProfileTag>, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let tags = sqlx::query_as!(
            ProfileTagSqlx,
            r#"
            SELECT pt.*
            FROM profile_tag pt
            JOIN join_user_profile_tag jpt ON pt.id = jpt.profile_tag_id
            WHERE jpt.user_profile_id = $1
            "#,
            profile_id.as_i64()
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(tags.into_iter().map(|tag| tag.into()).collect())
    }

    #[tracing::instrument(skip(conn))]
    async fn add_pictures<'a, A>(conn: A, profile_id: Snowflake, picture_hashes: Vec<String>) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        sqlx::query!(
            r#"
            UPDATE user_profile
            SET picture_hashes = array_cat(picture_hashes, $2)
            WHERE id = $1
            "#,
            profile_id.as_i64(),
            &picture_hashes
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(conn))]
    async fn remove_pictures<'a, A>(
        conn: A,
        profile_id: Snowflake,
        picture_hashes: Vec<String>,
    ) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        sqlx::query!(
            r#"
            UPDATE user_profile
            SET picture_hashes = (
                SELECT array_agg(elem)
                FROM unnest(picture_hashes) elem
                WHERE elem <> ALL($2)
            )
            WHERE id = $1
            "#,
            profile_id.as_i64(),
            &picture_hashes
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    async fn is_profile_hash_used<'a, A>(conn: A, hash: &str) -> sqlx::Result<bool, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let result = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM user_profile
                WHERE avatar_hash = $1 OR $1 = ANY(picture_hashes)
            )
            "#,
            hash
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.exists.unwrap_or(false))
    }

    #[tracing::instrument(skip(conn))]
    async fn add_tag<'a, A>(conn: A, profile_id: Snowflake, tag_id: Snowflake) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let join_id = Snowflake::new();

        sqlx::query!(
            r#"
            INSERT INTO join_user_profile_tag (id, user_profile_id, profile_tag_id)
            VALUES ($1, $2, $3)
            "#,
            join_id.as_i64(),
            profile_id.as_i64(),
            tag_id.as_i64()
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(conn))]
    async fn remove_tag<'a, A>(conn: A, profile_id: Snowflake, tag_id: Snowflake) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        sqlx::query!(
            r#"
            DELETE FROM join_user_profile_tag
            WHERE user_profile_id = $1 AND profile_tag_id = $2
            "#,
            profile_id.as_i64(),
            tag_id.as_i64()
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(conn))]
    async fn bulk_add_tags<'a, A>(conn: A, profile_id: Snowflake, tag_ids: Vec<Snowflake>) -> Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        if tag_ids.is_empty() {
            return Ok(());
        }

        let mut query_builder =
            QueryBuilder::<Postgres>::new("INSERT INTO join_user_profile_tag (id, user_profile_id, profile_tag_id) ");

        query_builder.push("VALUES ");

        query_builder.push_values(tag_ids.iter(), |mut b, tag_id| {
            let join_id = Snowflake::new();
            b.push_bind(join_id.as_i64())
                .push_bind(profile_id.as_i64())
                .push_bind(tag_id.as_i64());
        });

        let query = query_builder.build();
        query.execute(&mut *conn).await?;

        Ok(())
    }

    #[tracing::instrument(skip(conn))]
    async fn bulk_remove_tags<'a, A>(conn: A, profile_id: Snowflake, tag_ids: Vec<Snowflake>) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        if tag_ids.is_empty() {
            return Ok(());
        }

        sqlx::query!(
            r#"
            DELETE FROM join_user_profile_tag
            WHERE user_profile_id = $1 AND profile_tag_id = ANY($2)
            "#,
            profile_id.as_i64(),
            &tag_ids.iter().map(|tag_id| tag_id.as_i64()).collect::<Vec<_>>()
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}
