use async_trait::async_trait;

use crate::domain::entities::chat::{Channel, ChannelParticipant, Message};
use crate::domain::errors::channel_error::ChannelError;
use crate::domain::errors::message_error::MessageError;
use crate::domain::repositories::chat::channel_repository::ChannelQueryParams;
use crate::domain::repositories::chat::message_repository::MessageQueryParams;
use crate::shared::types::snowflake::Snowflake;

#[async_trait]
pub trait ChatService: 'static + Sync + Send {
    // Channel methods
    async fn create_dm_channel(&self, profile1_id: Snowflake, profile2_id: Snowflake) -> Result<(), ChannelError>;

    async fn get_user_channels(
        &self,
        profile_id: Snowflake,
        params: &ChannelQueryParams,
    ) -> Result<Vec<Channel>, ChannelError>;

    async fn get_channel(&self, channel_id: Snowflake) -> Result<Channel, ChannelError>;
    async fn get_channel_participants(&self, channel_id: Snowflake) -> Result<Vec<ChannelParticipant>, ChannelError>;
    async fn is_channel_participant(&self, channel_id: Snowflake, profile_id: Snowflake) -> Result<bool, ChannelError>;

    // Message methods
    async fn send_message(
        &self,
        channel_id: Snowflake,
        author_id: Snowflake,
        content: &str,
    ) -> Result<(), MessageError>;

    async fn edit_message(&self, message_id: Snowflake, new_content: &str) -> Result<(), MessageError>;

    async fn get_channel_message(&self, message_id: Snowflake) -> Result<Message, MessageError>;

    async fn get_channel_messages(
        &self,
        channel_id: Snowflake,
        params: &MessageQueryParams,
    ) -> Result<Vec<Message>, MessageError>;

    async fn soft_delete_message(&self, message_id: Snowflake) -> Result<(), MessageError>;
    async fn hard_delete_message(&self, message_id: Snowflake) -> Result<(), MessageError>;
}
