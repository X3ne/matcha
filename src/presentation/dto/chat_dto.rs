use apistos::ApiComponent;
use garde::Validate;
use geo_types::Point;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::domain::entities::chat::{Channel, ChannelParticipant, Message};
use crate::domain::entities::user_profile::UserProfile;
use crate::domain::repositories::chat::channel_repository::{ChannelQueryParams, ChannelSortBy};
use crate::domain::repositories::chat::message_repository::{MessageQueryParams, MessageSortBy};
use crate::domain::repositories::repository::DEFAULT_LIMIT;
use crate::domain::repositories::user_profile_repo::{UserProfileQueryParams, UserProfileSortBy};
use crate::shared::types::filtering::SortOrder;
use crate::shared::types::snowflake::Snowflake;
use crate::shared::utils::build_cdn_profile_image_uri;

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[serde(rename(deserialize = "ChannelParticipant"))]
pub struct ChannelParticipantDto {
    pub id: Snowflake,
    pub name: String,
    pub avatar: Option<String>,
}

impl From<ChannelParticipant> for ChannelParticipantDto {
    fn from(participant: ChannelParticipant) -> Self {
        Self {
            id: participant.id,
            name: participant.name,
            avatar: participant.avatar_hash.map(|hash| build_cdn_profile_image_uri(&hash)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[serde(rename(deserialize = "Channel"))]
pub struct ChannelDto {
    pub id: Snowflake,
    pub name: String,
    pub participants: Vec<ChannelParticipantDto>,
}

impl ChannelDto {
    pub fn append_participants(&mut self, participant: Vec<ChannelParticipant>) {
        self.participants = participant.into_iter().map(ChannelParticipantDto::from).collect();
    }
}

impl From<Channel> for ChannelDto {
    fn from(channel: Channel) -> Self {
        Self {
            id: channel.id,
            name: channel.name,
            participants: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[serde(rename(deserialize = "MessageAuthor"))]
pub struct MessageAuthorDto {
    pub id: Snowflake,
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[serde(rename(deserialize = "Message"))]
pub struct MessageDto {
    pub id: Snowflake,
    pub author: MessageAuthorDto,
    pub channel_id: Snowflake,
    pub content: String,
    pub sent_at: chrono::NaiveDateTime,
    pub edited_at: Option<chrono::NaiveDateTime>,
}

impl From<Message> for MessageDto {
    fn from(value: Message) -> Self {
        let author_dto = MessageAuthorDto {
            id: value.author_id,
            name: value.author_name,
            avatar: value.author_avatar_hash.map(|hash| build_cdn_profile_image_uri(&hash)),
        };

        Self {
            id: value.id,
            author: author_dto,
            channel_id: value.channel_id,
            content: value.content,
            sent_at: value.sent_at,
            edited_at: value.edited_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Validate)]
#[serde(rename(deserialize = "PostMessage"))]
pub struct PostMessageDto {
    #[garde(length(min = 1, max = 2000))]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Validate)]
#[serde(rename(deserialize = "ChannelQueryParams"))]
pub struct ChannelQueryParamsDto {
    #[garde(range(min = 1, max = 100))]
    #[serde(default = "limit_default")]
    pub limit: i64,
    #[garde(range(min = 0))]
    #[serde(default)]
    pub offset: i64,

    #[garde(skip)]
    pub sort_by: Option<ChannelSortBy>,
    #[garde(skip)]
    pub sort_order: Option<SortOrder>,
}

fn limit_default() -> i64 {
    DEFAULT_LIMIT.unwrap_or(50)
}

impl Into<ChannelQueryParams> for ChannelQueryParamsDto {
    fn into(self) -> ChannelQueryParams {
        ChannelQueryParams {
            limit: Some(self.limit),
            offset: Some(self.offset),

            sort_by: self.sort_by,
            sort_order: self.sort_order,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Validate)]
#[serde(rename(deserialize = "MessageQueryParams"))]
pub struct MessageQueryParamsDto {
    #[garde(range(min = 1, max = 100))]
    #[serde(default = "limit_default")]
    pub limit: i64,
    #[garde(range(min = 0))]
    #[serde(default)]
    pub offset: i64,

    #[garde(skip)]
    pub sort_by: Option<MessageSortBy>,
    #[garde(skip)]
    pub sort_order: Option<SortOrder>,
}

impl Into<MessageQueryParams> for MessageQueryParamsDto {
    fn into(self) -> MessageQueryParams {
        MessageQueryParams {
            limit: Some(self.limit),
            offset: Some(self.offset),

            sort_by: self.sort_by,
            sort_order: self.sort_order,
        }
    }
}
