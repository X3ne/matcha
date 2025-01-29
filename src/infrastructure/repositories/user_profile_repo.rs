use async_trait::async_trait;
use geozero::wkb;
use sqlx::{Acquire, Error, Postgres, QueryBuilder};

use crate::domain::entities::profile_tag::ProfileTag;
use crate::domain::entities::user_profile::UserProfile;
use crate::domain::repositories::user_profile_repo::{
    UserProfileQueryParams, UserProfileRepository, UserProfileSortBy,
};
use crate::infrastructure::models::profile_tag::ProfileTagSqlx;
use crate::infrastructure::models::user_profile::{
    RecommendedUserProfile, UserProfileInsert, UserProfileSqlx, UserProfileUpdate,
};
use crate::shared::types::snowflake::Snowflake;
use crate::shared::types::user_profile::{Gender, Orientation};

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
            INSERT INTO user_profile (id, user_id, name, avatar_hash, picture_hashes, bio, birth_date, gender, sexual_orientation, location, min_age, max_age, max_distance_km)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8::gender, $9::sexual_orientation, $10::geometry, $11, $12, $13)
            "#,
            id.as_i64(),
            profile.user_id.as_i64(),
            profile.name,
            profile.avatar_hash,
            &profile.picture_hashes,
            profile.bio,
            profile.birth_date,
            profile.gender as _,
            profile.sexual_orientation as _,
            wkb::Encode(geom) as _,
            profile.min_age,
            profile.max_age,
            profile.max_distance_km
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
                birth_date,
                gender AS "gender: _",
                sexual_orientation AS "sexual_orientation: _",
                min_age,
                max_age,
                max_distance_km,
                location AS "location!: _",
                rating,
                last_active,
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
                birth_date,
                gender AS "gender: _",
                sexual_orientation AS "sexual_orientation: _",
                min_age,
                max_age,
                max_distance_km,
                location AS "location!: _",
                rating,
                last_active,
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
                gender = COALESCE($5, gender),
                sexual_orientation = COALESCE($6, sexual_orientation),
                location = COALESCE($7, location),
                min_age = COALESCE($8, min_age),
                max_age = COALESCE($9, max_age),
                max_distance_km = COALESCE($10, max_distance_km)
            WHERE
                id = $1
            "#,
            id.as_i64(),
            profile.name,
            profile.avatar_hash,
            profile.bio,
            profile.gender as _,
            profile.sexual_orientation as _,
            encode as _,
            profile.min_age,
            profile.max_age,
            profile.max_distance_km
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(conn))]
    async fn search<'a, A>(
        conn: A,
        params: &UserProfileQueryParams,
        current_profile_id: Snowflake,
    ) -> sqlx::Result<Vec<UserProfile>, Error>
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
            query_builder.push("COUNT(DISTINCT pt.id) AS common_tags_count, ");
        } else {
            query_builder.push("0 AS common_tags_count, ");
        }

        query_builder.push("EXTRACT(YEAR FROM AGE(up.birth_date)) AS age ");

        query_builder.push(
            " FROM user_profile up
        LEFT JOIN join_user_profile_tag jpt ON up.id = jpt.user_profile_id
        LEFT JOIN profile_tag pt ON jpt.profile_tag_id = pt.id
        WHERE 1=1",
        );

        // exclude current user
        query_builder.push(" AND up.id != ");
        query_builder.push_bind(current_profile_id);

        if let Some(min_age) = params.min_age {
            query_builder.push(" AND EXTRACT(YEAR FROM AGE(up.birth_date)) >= ");
            query_builder.push_bind(min_age);
        }
        if let Some(max_age) = params.max_age {
            query_builder.push(" AND EXTRACT(YEAR FROM AGE(up.birth_date)) <= ");
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
                query_builder.push(" ORDER BY EXTRACT(YEAR FROM AGE(up.birth_date))");
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
    async fn recommend<'a, A>(
        conn: A,
        user_id: Snowflake,
        location: geo_types::Geometry<f64>,
        max_distance_km: f64,
        gender: Gender,
        orientation: Orientation,
        birth_date: chrono::NaiveDate,
        min_age: u8,
        max_age: u8,
    ) -> sqlx::Result<Vec<UserProfile>, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;
        let wkb_location = wkb::Encode(location);

        let profiles = sqlx::query_as!(
            RecommendedUserProfile,
            r#"
            WITH user_input AS (
                SELECT
                    $1::BIGINT AS user_id,
                    $2::gender AS user_gender,
                    $3::sexual_orientation AS user_orientation,
                    ST_SetSRID(ST_GeomFromEWKB($4), 4326) AS user_location,
                    $5::INT AS min_age,
                    $6::INT AS max_age,
                    $7::INT AS max_distance_km,
                    $8::DATE AS birth_date
            )
            SELECT up.id, up.user_id, up.name, up.avatar_hash, up.picture_hashes,
                   up.bio, up.birth_date, up.gender AS "gender: Gender",
                   up.sexual_orientation AS "sexual_orientation: Orientation",
                   up.min_age, up.max_age, up.max_distance_km,
                   up.location AS "location!: _",
                   up.rating, up.last_active, up.created_at, up.updated_at,
                   subquery.distance, subquery.common_tags_count, subquery.inactivity_duration,
                   (
                       (CASE
                            WHEN (ui.user_gender = 'male' AND up.gender = 'female' AND (up.sexual_orientation = 'male' OR up.sexual_orientation = 'bisexual')) THEN 1.5
                            WHEN (ui.user_gender = 'male' AND ui.user_orientation = 'bisexual' AND (up.gender = 'female' OR (up.gender = 'male' AND up.sexual_orientation = 'bisexual'))) THEN 1.5
                            WHEN (ui.user_gender = 'female' AND ui.user_orientation = 'male' AND up.gender = 'male' AND (up.sexual_orientation = 'female' OR up.sexual_orientation = 'bisexual')) THEN 1.5
                            WHEN (ui.user_gender = 'female' AND ui.user_orientation = 'bisexual' AND (up.gender = 'male' OR (up.gender = 'female' AND up.sexual_orientation = 'bisexual'))) THEN 1.5
                            WHEN (ui.user_gender = 'female' AND ui.user_orientation = 'female' AND up.gender = 'female' AND (up.sexual_orientation = 'female' OR up.sexual_orientation = 'bisexual')) THEN 1.5
                            WHEN (ui.user_gender = 'male' AND ui.user_orientation = 'male' AND up.gender = 'male' AND (up.sexual_orientation = 'male' OR up.sexual_orientation = 'bisexual')) THEN 1.5
                            ELSE 1
                       END) *
                       (1.0 / (subquery.distance + 1)) *
                       (subquery.common_tags_count + 1) *
                       (1 / (subquery.inactivity_duration + 1)) *
                       (up.rating + 1) *
                       (CASE WHEN up.avatar_hash IS NOT NULL THEN 1.2 ELSE 1 END) *
                       (CASE WHEN up.bio IS NOT NULL AND LENGTH(up.bio) > 10 THEN 1.3 ELSE 1 END)
                   ) AS recommendation_score
            FROM (
                SELECT
                    up.id,
                    ST_Distance(up.location::geography, ui.user_location::geography) AS distance,
                    COUNT(DISTINCT pt.id) AS common_tags_count,
                    EXTRACT(EPOCH FROM (NOW() - up.last_active)) AS inactivity_duration
                FROM user_profile up
                CROSS JOIN user_input ui
                LEFT JOIN join_user_profile_tag jpt ON up.id = jpt.user_profile_id
                LEFT JOIN profile_tag pt ON jpt.profile_tag_id = pt.id
                LEFT JOIN profile_like pl ON up.id = pl.liked_user_profile_id AND pl.user_profile_id = ui.user_id
                WHERE
                    pl.id IS NULL
                    AND up.id <> ui.user_id
                    AND (
                        (ui.user_gender = 'male' AND up.gender = 'female' AND (up.sexual_orientation = 'male' OR up.sexual_orientation = 'bisexual')) OR
                        (ui.user_gender = 'male' AND ui.user_orientation = 'bisexual' AND (up.gender = 'female' OR (up.gender = 'male' AND up.sexual_orientation = 'bisexual'))) OR
                        (ui.user_gender = 'female' AND ui.user_orientation = 'male' AND up.gender = 'male' AND (up.sexual_orientation = 'female' OR up.sexual_orientation = 'bisexual')) OR
                        (ui.user_gender = 'female' AND ui.user_orientation = 'bisexual' AND (up.gender = 'male' OR (up.gender = 'female' AND up.sexual_orientation = 'bisexual'))) OR
                        (ui.user_gender = 'female' AND ui.user_orientation = 'female' AND up.gender = 'female' AND (up.sexual_orientation = 'female' OR up.sexual_orientation = 'bisexual')) OR
                        (ui.user_gender = 'male' AND ui.user_orientation = 'male' AND up.gender = 'male' AND (up.sexual_orientation = 'male' OR up.sexual_orientation = 'bisexual'))
                    )
                    AND ST_DWithin(up.location::geography, ui.user_location::geography, LEAST(ui.max_distance_km, up.max_distance_km) * 1000)
                    AND EXTRACT(YEAR FROM AGE(NOW(), up.birth_date)) BETWEEN ui.min_age AND ui.max_age
                    AND EXTRACT(YEAR FROM AGE(NOW(), ui.birth_date)) BETWEEN up.min_age AND up.max_age
                GROUP BY up.id, ui.user_location
            ) AS subquery
            JOIN user_profile up ON up.id = subquery.id
            CROSS JOIN user_input ui
            ORDER BY recommendation_score DESC
            LIMIT 10;
            "#,
            user_id.as_i64(),
            gender as _,
            orientation as _,
            wkb_location as _,
            min_age as i32,
            max_age as i32,
            max_distance_km as _,
            birth_date as _
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(profiles.into_iter().map(|profile| profile.into()).collect())
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

    #[tracing::instrument(skip(conn))]
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

    #[tracing::instrument(skip(conn))]
    async fn add_like<'a, A>(conn: A, profile_id: Snowflake, liked_profile_id: Snowflake) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let like_id = Snowflake::new();

        sqlx::query!(
            r#"
            INSERT INTO profile_like (id, user_profile_id, liked_user_profile_id)
            VALUES ($1, $2, $3)
            "#,
            like_id.as_i64(),
            profile_id.as_i64(),
            liked_profile_id.as_i64()
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(conn))]
    async fn remove_like<'a, A>(conn: A, profile_id: Snowflake, liked_profile_id: Snowflake) -> Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let result = sqlx::query!(
            r#"
            DELETE FROM profile_like
            WHERE user_profile_id = $1 AND liked_user_profile_id = $2
            RETURNING user_profile_id
            "#,
            profile_id.as_i64(),
            liked_profile_id.as_i64()
        )
        .fetch_optional(&mut *conn)
        .await?;

        if result.is_none() {
            return Err(Error::RowNotFound);
        }

        Ok(())
    }

    #[tracing::instrument(skip(conn))]
    async fn is_like_exists<'a, A>(
        conn: A,
        profile_id: Snowflake,
        liked_profile_id: Snowflake,
    ) -> sqlx::Result<bool, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let result = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM profile_like
                WHERE user_profile_id = $1 AND liked_user_profile_id = $2
            )
            "#,
            profile_id.as_i64(),
            liked_profile_id.as_i64()
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.exists.unwrap_or(false))
    }

    #[tracing::instrument(skip(conn))]
    async fn is_match_exists<'a, A>(
        conn: A,
        profile_id: Snowflake,
        matched_profile_id: Snowflake,
    ) -> sqlx::Result<bool, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let result = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM profile_like pl1
                JOIN profile_like pl2 ON pl1.user_profile_id = pl2.liked_user_profile_id AND pl1.liked_user_profile_id = pl2.user_profile_id
                WHERE pl1.user_profile_id = $1 AND pl1.liked_user_profile_id = $2
            )
            "#,
            profile_id.as_i64(),
            matched_profile_id.as_i64()
        )
        .fetch_one(&mut *conn)
        .await?;

        Ok(result.exists.unwrap_or(false))
    }

    #[tracing::instrument(skip(conn))]
    async fn get_my_likes<'a, A>(conn: A, profile_id: Snowflake) -> sqlx::Result<Vec<UserProfile>, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let profiles = sqlx::query_as!(
            UserProfileSqlx,
            r#"
            SELECT
                up.id,
                up.user_id,
                up.name,
                up.avatar_hash,
                up.picture_hashes,
                up.bio,
                up.birth_date,
                up.gender AS "gender: _",
                up.sexual_orientation AS "sexual_orientation: _",
                up.min_age,
                up.max_age,
                up.max_distance_km,
                up.location AS "location!: _",
                up.rating,
                up.last_active,
                up.created_at,
                up.updated_at
            FROM user_profile up
            JOIN profile_like pl ON up.id = pl.liked_user_profile_id
            WHERE pl.user_profile_id = $1
            "#,
            profile_id.as_i64()
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(profiles.into_iter().map(|profile| profile.into()).collect())
    }

    #[tracing::instrument(skip(conn))]
    async fn get_profile_likes<'a, A>(conn: A, profile_id: Snowflake) -> sqlx::Result<Vec<UserProfile>, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let profiles = sqlx::query_as!(
            UserProfileSqlx,
            r#"
            SELECT
                up.id,
                up.user_id,
                up.name,
                up.avatar_hash,
                up.picture_hashes,
                up.bio,
                up.birth_date,
                up.gender AS "gender: _",
                up.sexual_orientation AS "sexual_orientation: _",
                up.min_age,
                up.max_age,
                up.max_distance_km,
                up.location AS "location!: _",
                up.rating,
                up.last_active,
                up.created_at,
                up.updated_at
            FROM user_profile up
            JOIN profile_like pl ON up.id = pl.user_profile_id
            WHERE pl.liked_user_profile_id = $1
            "#,
            profile_id.as_i64()
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(profiles.into_iter().map(|profile| profile.into()).collect())
    }

    #[tracing::instrument(skip(conn))]
    async fn get_matches<'a, A>(conn: A, profile_id: Snowflake) -> sqlx::Result<Vec<UserProfile>, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let profiles = sqlx::query_as!(
            UserProfileSqlx,
            r#"
            SELECT
                up.id,
                up.user_id,
                up.name,
                up.avatar_hash,
                up.picture_hashes,
                up.bio,
                up.birth_date,
                up.gender AS "gender: _",
                up.sexual_orientation AS "sexual_orientation: _",
                up.min_age,
                up.max_age,
                up.max_distance_km,
                up.location AS "location!: _",
                up.rating,
                up.last_active,
                up.created_at,
                up.updated_at
            FROM user_profile up
            JOIN profile_like pl1 ON up.id = pl1.user_profile_id
            JOIN profile_like pl2 ON up.id = pl2.liked_user_profile_id
            WHERE pl1.liked_user_profile_id = $1 AND pl2.user_profile_id = $1
            "#,
            profile_id.as_i64()
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(profiles.into_iter().map(|profile| profile.into()).collect())
    }

    #[tracing::instrument(skip(conn))]
    async fn increase_fame_rating<'a, A>(conn: A, profile_id: Snowflake, rating: i32) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        sqlx::query!(
            r#"
            UPDATE user_profile
            SET rating = rating + $2
            WHERE id = $1
            "#,
            profile_id.as_i64(),
            rating
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(conn))]
    async fn decrease_fame_rating<'a, A>(conn: A, profile_id: Snowflake, rating: i32) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        sqlx::query!(
            r#"
            UPDATE user_profile
            SET rating = rating - $2
            WHERE id = $1
            "#,
            profile_id.as_i64(),
            rating
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(conn))]
    async fn view_profile<'a, A>(
        conn: A,
        profile_id: Snowflake,
        viewed_profile_id: Snowflake,
    ) -> sqlx::Result<(), Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let view_id = Snowflake::new();

        sqlx::query!(
            r#"
            INSERT INTO profile_view (id, user_profile_id, viewer_profile_id)
            VALUES ($1, $2, $3)
            "#,
            view_id.as_i64(),
            viewed_profile_id.as_i64(),
            profile_id.as_i64()
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(conn))]
    async fn get_viewers<'a, A>(conn: A, profile_id: Snowflake) -> sqlx::Result<Vec<UserProfile>, Error>
    where
        A: Acquire<'a, Database = Postgres> + Send,
    {
        let mut conn = conn.acquire().await?;

        let profiles = sqlx::query_as!(
            UserProfileSqlx,
            r#"
            SELECT
                up.id,
                up.user_id,
                up.name,
                up.avatar_hash,
                up.picture_hashes,
                up.bio,
                up.birth_date,
                up.gender AS "gender: _",
                up.sexual_orientation AS "sexual_orientation: _",
                up.min_age,
                up.max_age,
                up.max_distance_km,
                up.location AS "location!: _",
                up.rating,
                up.last_active,
                up.created_at,
                up.updated_at
            FROM user_profile up
            JOIN profile_view pv ON up.id = pv.viewer_profile_id
            WHERE pv.user_profile_id = $1
            "#,
            profile_id.as_i64()
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(profiles.into_iter().map(|profile| profile.into()).collect())
    }
}
