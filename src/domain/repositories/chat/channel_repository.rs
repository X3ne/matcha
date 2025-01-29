use std::fmt::Display;

use apistos::ApiComponent;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::Acquire;

use crate::domain::entities::chat::{Channel, ChannelParticipant};
use crate::domain::repositories::repository::{QueryParams, DEFAULT_LIMIT, DEFAULT_OFFSET};
use crate::infrastructure::models::chat::{ChannelInsert, ChannelUpdate};
use crate::shared::types::filtering::SortOrder;
use crate::shared::types::snowflake::Snowflake;

#[derive(Debug, Default)]
pub struct ChannelQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub sort_by: Option<ChannelSortBy>,
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[serde(rename_all = "snake_case")]
pub enum ChannelSortBy {
    CreatedAt,
    LastActivity,
}

impl Display for ChannelSortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelSortBy::CreatedAt => write!(f, "created_at"),
            ChannelSortBy::LastActivity => write!(f, "last_activity"),
        }
    }
}

impl ChannelQueryParams {
    pub fn sort_by(&self) -> String {
        self.sort_by.clone().unwrap_or(ChannelSortBy::LastActivity).to_string()
    }

    pub fn sort_order(&self) -> String {
        self.sort_order
            .clone()
            .unwrap_or(SortOrder::Desc)
            .to_string()
            .to_uppercase()
    }
}

impl QueryParams for ChannelQueryParams {
    fn limit(&self) -> i64 {
        self.limit.unwrap_or(DEFAULT_LIMIT.unwrap_or(50))
    }
    fn offset(&self) -> i64 {
        self.offset.unwrap_or(DEFAULT_OFFSET.unwrap_or(0))
    }
}

#[async_trait]
pub trait ChannelRepository<Db>: Send + Sync {
    async fn insert<'a, A>(conn: A, channel: &ChannelInsert) -> sqlx::Result<Channel, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_by_id<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<Channel, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_profile_channels<'a, A>(
        conn: A,
        profile_id: Snowflake,
        params: &ChannelQueryParams,
    ) -> sqlx::Result<Vec<Channel>, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_participants<'a, A>(
        conn: A,
        channel_id: Snowflake,
    ) -> sqlx::Result<Vec<ChannelParticipant>, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn add_participant<'a, A>(
        conn: A,
        channel_id: Snowflake,
        profile_id: Snowflake,
    ) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn add_participants<'a, A>(
        conn: A,
        channel_id: Snowflake,
        profile_ids: Vec<Snowflake>,
    ) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn update<'a, A>(conn: A, id: Snowflake, channel: &ChannelUpdate) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn delete<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;
}
