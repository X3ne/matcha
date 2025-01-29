use crate::infrastructure::models::chat::{ChannelParticipantSqlx, ChannelSqlx, MessageSqlx};
use crate::shared::types::snowflake::Snowflake;

#[derive(Debug)]
pub struct Channel {
    pub id: Snowflake,
    pub name: String,
    pub last_activity: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

impl From<ChannelSqlx> for Channel {
    fn from(channel: ChannelSqlx) -> Self {
        Self {
            id: channel.id,
            name: channel.name,
            last_activity: channel.last_activity,
            created_at: channel.created_at,
        }
    }
}

#[derive(Debug)]
pub struct Message {
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

impl From<MessageSqlx> for Message {
    fn from(message: MessageSqlx) -> Self {
        Self {
            id: message.id,
            author_id: message.author_id,
            author_name: message.author_name,
            author_avatar_hash: message.author_avatar_hash,
            channel_id: message.channel_id,
            content: message.content,
            deleted: message.deleted,
            sent_at: message.sent_at,
            edited_at: message.edited_at,
        }
    }
}

#[derive(Debug)]
pub struct ChannelParticipant {
    pub id: Snowflake,
    pub profile_id: Snowflake,
    pub channel_id: Snowflake,
    pub name: String,
    pub avatar_hash: Option<String>,
    pub joined_at: chrono::NaiveDateTime,
}

impl From<ChannelParticipantSqlx> for ChannelParticipant {
    fn from(participant: ChannelParticipantSqlx) -> Self {
        Self {
            id: participant.id,
            profile_id: participant.profile_id,
            channel_id: participant.channel_id,
            name: participant.name,
            avatar_hash: participant.avatar_hash,
            joined_at: participant.joined_at,
        }
    }
}
