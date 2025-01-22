use crate::domain::entities::profile_tag::ProfileTag;
use crate::domain::entities::user_profile::UserProfile;
use crate::domain::repositories::repository::{QueryParams, DEFAULT_LIMIT, DEFAULT_OFFSET};
use crate::infrastructure::models::user_profile::{UserProfileInsert, UserProfileUpdate};
use crate::shared::types::filtering::SortOrder;
use crate::shared::types::snowflake::Snowflake;
use crate::shared::types::user_profile::{Gender, Orientation};
use apistos::ApiComponent;
use async_trait::async_trait;
use geo_types::Point;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::Acquire;
use std::fmt::Display;

#[derive(Debug)]
pub struct UserProfileQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub min_age: Option<i32>,
    pub max_age: Option<i32>,
    pub min_fame_rating: Option<i32>,
    pub max_fame_rating: Option<i32>,
    pub location: Option<Point>,
    pub radius_km: Option<f64>,
    pub tag_ids: Option<Vec<Snowflake>>,
    pub sort_by: Option<UserProfileSortBy>,
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[serde(rename_all = "snake_case")]
pub enum UserProfileSortBy {
    Age,
    FameRating,
    Distance,
    Tags, // TODO: Implement tags sorting
}

impl Display for UserProfileSortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserProfileSortBy::Age => write!(f, "age"),
            UserProfileSortBy::FameRating => write!(f, "rating"),
            UserProfileSortBy::Distance => write!(f, "distance"),
            UserProfileSortBy::Tags => write!(f, "tags"),
        }
    }
}

impl QueryParams for UserProfileQueryParams {
    fn limit(&self) -> i64 {
        self.limit.unwrap_or(DEFAULT_LIMIT.unwrap_or(50))
    }
    fn offset(&self) -> i64 {
        self.offset.unwrap_or(DEFAULT_OFFSET.unwrap_or(0))
    }
}

#[async_trait]
pub trait UserProfileRepository<Db>: Send + Sync {
    async fn insert<'a, A>(conn: A, profile: &UserProfileInsert) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_by_id<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<UserProfile, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_by_user_id<'a, A>(conn: A, user_id: Snowflake) -> sqlx::Result<UserProfile, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn update<'a, A>(conn: A, id: Snowflake, profile: &UserProfileUpdate) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn search<'a, A>(
        conn: A,
        params: &UserProfileQueryParams,
        current_profile_id: Snowflake,
    ) -> sqlx::Result<Vec<UserProfile>, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn recommend<'a, A>(
        conn: A,
        user_id: Snowflake,
        location: geo_types::Geometry<f64>,
        radius_km: f64,
        gender: Gender,
        orientation: Orientation,
        min_age: u8,
        max_age: u8,
    ) -> sqlx::Result<Vec<UserProfile>, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_profile_tags<'a, A>(conn: A, profile_id: Snowflake) -> sqlx::Result<Vec<ProfileTag>, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn add_pictures<'a, A>(
        conn: A,
        profile_id: Snowflake,
        picture_hashes: Vec<String>,
    ) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn remove_pictures<'a, A>(
        conn: A,
        profile_id: Snowflake,
        picture_hashes: Vec<String>,
    ) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn is_profile_hash_used<'a, A>(conn: A, hash: &str) -> sqlx::Result<bool, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn add_tag<'a, A>(conn: A, profile_id: Snowflake, tag_id: Snowflake) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn remove_tag<'a, A>(conn: A, profile_id: Snowflake, tag_id: Snowflake) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn bulk_add_tags<'a, A>(
        conn: A,
        profile_id: Snowflake,
        tag_ids: Vec<Snowflake>,
    ) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn bulk_remove_tags<'a, A>(
        conn: A,
        profile_id: Snowflake,
        tag_ids: Vec<Snowflake>,
    ) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn add_like<'a, A>(
        conn: A,
        profile_id: Snowflake,
        liked_profile_id: Snowflake,
    ) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn remove_like<'a, A>(
        conn: A,
        profile_id: Snowflake,
        liked_profile_id: Snowflake,
    ) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn is_like_exists<'a, A>(
        conn: A,
        profile_id: Snowflake,
        liked_profile_id: Snowflake,
    ) -> sqlx::Result<bool, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn is_match_exists<'a, A>(
        conn: A,
        profile_id: Snowflake,
        matched_profile_id: Snowflake,
    ) -> sqlx::Result<bool, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_my_likes<'a, A>(conn: A, profile_id: Snowflake) -> sqlx::Result<Vec<UserProfile>, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_profile_likes<'a, A>(conn: A, profile_id: Snowflake) -> sqlx::Result<Vec<UserProfile>, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_matches<'a, A>(conn: A, profile_id: Snowflake) -> sqlx::Result<Vec<UserProfile>, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn increase_fame_rating<'a, A>(conn: A, profile_id: Snowflake, rating: i32) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn decrease_fame_rating<'a, A>(conn: A, profile_id: Snowflake, rating: i32) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;
}
