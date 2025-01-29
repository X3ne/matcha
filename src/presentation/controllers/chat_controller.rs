use std::sync::Arc;

use actix_web::web;
use apistos::actix::NoContent;
use apistos::api_operation;
use garde::Validate;

use crate::domain::errors::channel_error::ChannelError;
use crate::domain::services::chat_service::ChatService;
use crate::domain::services::user_profile_service::UserProfileService;
use crate::infrastructure::error::ApiError;
use crate::infrastructure::gateway::events::GatewayEvent;
use crate::infrastructure::gateway::Gateway;
use crate::presentation::dto::chat_dto::{MessageDto, MessageQueryParamsDto, PostMessageDto};
use crate::presentation::extractors::auth_extractor::Session;
use crate::shared::types::peer_infos::PeerInfos;
use crate::shared::types::snowflake::Snowflake;
use crate::shared::utils::build_cdn_profile_image_uri;

#[api_operation(
    tag = "chat",
    operation_id = "post_channel_message",
    summary = "Post a message to a channel",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(chat_service, user_profile_service, gateway, session))]
pub async fn post_channel_message(
    chat_service: web::Data<Arc<dyn ChatService>>,
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    gateway: web::Data<Arc<Gateway>>,
    channel_id: web::Path<Snowflake>,
    body: web::Json<PostMessageDto>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;
    let profile = user_profile_service.get_by_user_id(user.id).await?;

    let channel_id = channel_id.into_inner();
    let body = body.into_inner();
    body.validate()?;

    let _ = chat_service.get_channel(channel_id).await?;

    let _ = chat_service.send_message(channel_id, profile.id, &body.content).await?;

    for participant in chat_service.get_channel_participants(channel_id).await? {
        if participant.profile_id != profile.id {
            gateway
                .send_event(
                    &participant.profile_id,
                    &GatewayEvent::MessageReceived {
                        channel_id,
                        sender_id: profile.id,
                        sender_username: profile.name.clone(),
                        sender_avatar: profile
                            .avatar_hash
                            .clone()
                            .map(|hash| build_cdn_profile_image_uri(&hash)),
                        content: body.content.clone(),
                    },
                )
                .await;
        }
    }

    Ok(NoContent)
}

#[api_operation(
    tag = "chat",
    operation_id = "get_channel_message",
    summary = "Retrieve a specific message from a channel",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(chat_service, user_profile_service, session))]
pub async fn get_channel_message(
    chat_service: web::Data<Arc<dyn ChatService>>,
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    path_params: web::Path<(Snowflake, Snowflake)>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<web::Json<MessageDto>, ApiError> {
    let user = session.authenticated_user()?;
    let profile = user_profile_service.get_by_user_id(user.id).await?;

    let channel_id = path_params.0;
    let message_id = path_params.1;

    if !chat_service.is_channel_participant(channel_id, profile.id).await? {
        return Err(ChannelError::NotChannelParticipant.into());
    }

    let _ = chat_service.get_channel(channel_id).await?;

    let message = chat_service.get_channel_message(message_id).await?;

    Ok(web::Json(message.into()))
}

#[api_operation(
    tag = "chat",
    operation_id = "get_channel_messages",
    summary = "Retrieve messages from a channel",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(chat_service, user_profile_service, session))]
pub async fn get_channel_messages(
    chat_service: web::Data<Arc<dyn ChatService>>,
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    channel_id: web::Path<Snowflake>,
    params: web::Query<MessageQueryParamsDto>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<web::Json<Vec<MessageDto>>, ApiError> {
    let user = session.authenticated_user()?;
    let profile = user_profile_service.get_by_user_id(user.id).await?;

    let channel_id = channel_id.into_inner();
    let params = params.into_inner();

    let _ = chat_service.get_channel(channel_id).await?;

    if !chat_service.is_channel_participant(channel_id, profile.id).await? {
        return Err(ChannelError::NotChannelParticipant.into());
    }

    let messages = chat_service.get_channel_messages(channel_id, &params.into()).await?;

    Ok(web::Json(messages.into_iter().map(Into::into).collect()))
}
