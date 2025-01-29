use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::chat::{Channel, ChannelParticipant, Message};
use crate::domain::errors::channel_error::ChannelError;
use crate::domain::errors::message_error::MessageError;
use crate::domain::repositories::chat::channel_repository::{ChannelQueryParams, ChannelRepository};
use crate::domain::repositories::chat::message_repository::{MessageQueryParams, MessageRepository};
use crate::domain::services::chat_service::ChatService;
use crate::infrastructure::models::chat::{ChannelInsert, MessageInsert, MessageUpdate};
use crate::infrastructure::repositories::chat::channel_repo::PgChannelRepository;
use crate::infrastructure::repositories::chat::message_repo::PgMessageRepository;
use crate::shared::types::snowflake::Snowflake;

#[derive(Clone)]
pub struct ChatServiceImpl {
    pub pool: Arc<PgPool>,
}

impl ChatServiceImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        ChatServiceImpl { pool }
    }
}

#[async_trait]
impl ChatService for ChatServiceImpl {
    #[tracing::instrument(skip(self))]
    async fn create_dm_channel(&self, profile1_id: Snowflake, profile2_id: Snowflake) -> Result<(), ChannelError> {
        let mut tx = self.pool.begin().await?;

        let channel_name = format!("dm-{}-{}", profile1_id, profile2_id);

        let channel = PgChannelRepository::insert(&mut *tx, &ChannelInsert { name: channel_name }).await?;

        PgChannelRepository::add_participants(&mut *tx, channel.id, vec![profile1_id, profile2_id]).await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn get_user_channels(
        &self,
        profile_id: Snowflake,
        params: &ChannelQueryParams,
    ) -> Result<Vec<Channel>, ChannelError> {
        let mut conn = self.pool.acquire().await?;

        let channels = PgChannelRepository::get_profile_channels(&mut *conn, profile_id, params).await?;

        Ok(channels)
    }

    #[tracing::instrument(skip(self))]
    async fn get_channel(&self, channel_id: Snowflake) -> Result<Channel, ChannelError> {
        let mut conn = self.pool.acquire().await?;

        let channel = PgChannelRepository::get_by_id(&mut *conn, channel_id).await?;

        Ok(channel)
    }

    #[tracing::instrument(skip(self))]
    async fn get_channel_participants(&self, channel_id: Snowflake) -> Result<Vec<ChannelParticipant>, ChannelError> {
        let mut conn = self.pool.acquire().await?;

        let participants = PgChannelRepository::get_participants(&mut *conn, channel_id).await?;

        Ok(participants)
    }

    #[tracing::instrument(skip(self))]
    async fn is_channel_participant(&self, channel_id: Snowflake, profile_id: Snowflake) -> Result<bool, ChannelError> {
        let mut conn = self.pool.acquire().await?;

        let participants = PgChannelRepository::get_participants(&mut *conn, channel_id).await?;

        Ok(participants.iter().any(|p| p.profile_id == profile_id))
    }

    #[tracing::instrument(skip(self))]
    async fn send_message(
        &self,
        channel_id: Snowflake,
        author_id: Snowflake,
        content: &str,
    ) -> Result<(), MessageError> {
        let mut tx = self.pool.begin().await?;

        PgMessageRepository::insert(
            &mut *tx,
            &MessageInsert {
                channel_id,
                author_id,
                content: content.to_string(),
                sent_at: chrono::Utc::now().naive_utc(),
            },
        )
        .await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn edit_message(&self, message_id: Snowflake, new_content: &str) -> Result<(), MessageError> {
        let mut tx = self.pool.begin().await?;

        PgMessageRepository::update(
            &mut *tx,
            message_id,
            &MessageUpdate {
                content: Some(new_content.to_string()),
                edited_at: Some(chrono::Utc::now().naive_utc()),
                ..Default::default()
            },
        )
        .await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn get_channel_message(&self, message_id: Snowflake) -> Result<Message, MessageError> {
        let mut conn = self.pool.acquire().await?;

        let message = PgMessageRepository::get_by_id(&mut *conn, message_id).await?;

        Ok(message)
    }

    #[tracing::instrument(skip(self))]
    async fn get_channel_messages(
        &self,
        channel_id: Snowflake,
        params: &MessageQueryParams,
    ) -> Result<Vec<Message>, MessageError> {
        let mut conn = self.pool.acquire().await?;

        let messages = PgMessageRepository::get_channel_messages(&mut *conn, channel_id, params).await?;

        Ok(messages)
    }

    #[tracing::instrument(skip(self))]
    async fn soft_delete_message(&self, message_id: Snowflake) -> Result<(), MessageError> {
        let mut tx = self.pool.begin().await?;

        PgMessageRepository::update(
            &mut *tx,
            message_id,
            &MessageUpdate {
                deleted: Some(true),
                ..Default::default()
            },
        )
        .await?;

        tx.commit().await?;

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn hard_delete_message(&self, message_id: Snowflake) -> Result<(), MessageError> {
        let mut tx = self.pool.begin().await?;

        PgMessageRepository::delete(&mut *tx, message_id).await?;

        tx.commit().await?;

        Ok(())
    }
}
