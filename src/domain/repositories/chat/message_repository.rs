use std::fmt::Display;

use apistos::ApiComponent;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::Acquire;

use crate::domain::entities::chat::Message;
use crate::domain::repositories::repository::{QueryParams, DEFAULT_LIMIT, DEFAULT_OFFSET};
use crate::infrastructure::models::chat::{MessageInsert, MessageUpdate};
use crate::shared::types::filtering::SortOrder;
use crate::shared::types::snowflake::Snowflake;

#[derive(Debug)]
pub struct MessageQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub sort_by: Option<MessageSortBy>,
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[serde(rename_all = "snake_case")]
pub enum MessageSortBy {
    SentAt,
}

impl Display for MessageSortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageSortBy::SentAt => write!(f, "sent_at"),
        }
    }
}

impl MessageQueryParams {
    pub fn sort_by(&self) -> String {
        self.sort_by.clone().unwrap_or(MessageSortBy::SentAt).to_string()
    }

    pub fn sort_order(&self) -> String {
        self.sort_order
            .clone()
            .unwrap_or(SortOrder::Asc)
            .to_string()
            .to_uppercase()
    }
}

impl QueryParams for MessageQueryParams {
    fn limit(&self) -> i64 {
        self.limit.unwrap_or(DEFAULT_LIMIT.unwrap_or(50))
    }
    fn offset(&self) -> i64 {
        self.offset.unwrap_or(DEFAULT_OFFSET.unwrap_or(0))
    }
}

#[async_trait]
pub trait MessageRepository<Db>: Send + Sync {
    async fn insert<'a, A>(conn: A, message: &MessageInsert) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_by_id<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<Message, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn get_channel_messages<'a, A>(
        conn: A,
        channel_id: Snowflake,
        params: &MessageQueryParams,
    ) -> sqlx::Result<Vec<Message>, sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn update<'a, A>(conn: A, id: Snowflake, message: &MessageUpdate) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;

    async fn delete<'a, A>(conn: A, id: Snowflake) -> sqlx::Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Db> + Send;
}
