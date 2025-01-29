use sqlx::FromRow;

use crate::shared::types::snowflake::Snowflake;

#[derive(FromRow, Debug, Clone)]
pub struct ChannelSqlx {
    pub id: Snowflake,
    pub name: String,
    pub last_activity: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct ChannelInsert {
    pub name: String,
}

#[derive(Debug)]
pub struct ChannelUpdate {
    pub name: Option<String>,
}

#[derive(FromRow, Debug, Clone)]
pub struct MessageSqlx {
    pub id: Snowflake,
    pub author_id: Snowflake,
    pub author_name: String,
    pub author_avatar_hash: Option<String>,
    pub channel_id: Snowflake,
    pub content: String,
    pub deleted: bool,
    pub sent_at: chrono::NaiveDateTime,
    pub edited_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug)]
pub struct MessageInsert {
    pub author_id: Snowflake,
    pub channel_id: Snowflake,
    pub content: String,
    pub sent_at: chrono::NaiveDateTime,
}

#[derive(Debug, Default)]
pub struct MessageUpdate {
    pub content: Option<String>,
    pub deleted: Option<bool>,
    pub edited_at: Option<chrono::NaiveDateTime>,
}

#[derive(FromRow, Debug, Clone)]
pub struct ChannelParticipantSqlx {
    pub id: Snowflake,
    pub profile_id: Snowflake,
    pub channel_id: Snowflake,
    pub name: String,
    pub avatar_hash: Option<String>,
    pub joined_at: chrono::NaiveDateTime,
}
