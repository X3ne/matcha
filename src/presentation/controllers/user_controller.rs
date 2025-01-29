use std::sync::Arc;

use actix_multipart::form::MultipartForm;
use actix_web::web;
use apistos::actix::NoContent;
use apistos::api_operation;
use futures::future::join_all;
use garde::{Error, Path, Report, Validate};
use geo_types::Point;

use crate::domain::constants::PROFILE_IMAGES_PATH;
use crate::domain::entities::chat::ChannelParticipant;
use crate::domain::errors::user_profile_error::UserProfileError;
use crate::domain::services::cdn_service::CdnService;
use crate::domain::services::chat_service::ChatService;
use crate::domain::services::user_profile_service::UserProfileService;
use crate::domain::services::user_service::UserService;
use crate::infrastructure::error::ApiError;
use crate::infrastructure::models::user_profile::UserProfileInsert;
use crate::infrastructure::services::iploc::locate_ip;
use crate::presentation::dto::chat_dto::ChannelDto;
use crate::presentation::dto::user_dto::{UpdateUserDto, UserDto};
use crate::presentation::dto::user_profile_dto::{CompleteOnboardingForm, UserProfileMeta};
use crate::presentation::extractors::auth_extractor::Session;
use crate::shared::types::peer_infos::PeerInfos;

#[api_operation(
    tag = "users",
    operation_id = "get_me",
    summary = "Get the current user",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(session))]
pub async fn get_me(session: Session, peer_infos: PeerInfos) -> Result<web::Json<UserDto>, ApiError> {
    let user = session.authenticated_user()?;

    Ok(web::Json(user.clone().into()))
}

#[api_operation(
    tag = "users",
    operation_id = "update_me",
    summary = "Update the current user",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_service, session))]
pub async fn update_me(
    user_service: web::Data<Arc<dyn UserService>>,
    body: web::Json<UpdateUserDto>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;

    let body = body.into_inner();
    body.validate()?;

    user_service.update(user.id, &body.into()).await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "users",
    operation_id = "complete_onboarding",
    summary = "Complete the onboarding process",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, cdn_service, session))]
pub async fn complete_onboarding(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    cdn_service: web::Data<Arc<dyn CdnService>>,
    MultipartForm(form): MultipartForm<CompleteOnboardingForm>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;

    if form.pictures.len() > 5 {
        return Err(UserProfileError::MaxImages.into());
    }

    let onboarding = form.profile.into_inner();
    onboarding.validate()?;

    if onboarding.avatar_index >= form.pictures.len() {
        // can't pass garde context to validate this (https://github.com/jprochazk/garde/issues/104)
        let mut report = Report::new();
        report.append(Path::new("avatar_index"), Error::new("Invalid avatar index"));
        return Err(ApiError::ValidationError(report));
    }

    let location = match onboarding.location {
        Some(location) => Point::new(location.latitude, location.longitude),
        None => {
            let ip_addr = peer_infos.ip_address.ok_or(ApiError::BadRequest("".to_string()))?;
            let location = locate_ip(ip_addr).await?;

            tracing::info!("Located IP: {:?}", location);

            Point::new(location.latitude, location.longitude)
        }
    };

    let mut picture_hashes = vec![];
    for mut file in form.pictures {
        let content_type = file
            .content_type
            .clone()
            .ok_or(ApiError::BadRequest("Missing content type".to_string()))?;

        if content_type.type_() != mime::IMAGE {
            return Err(ApiError::OnlyImagesAllowed);
        }

        let hash = cdn_service.upload_file(&mut file, PROFILE_IMAGES_PATH).await?;
        picture_hashes.push(hash);
    }

    user_profile_service
        .create(&UserProfileInsert {
            user_id: user.id,
            name: onboarding.name,
            avatar_hash: picture_hashes.get(onboarding.avatar_index).cloned(),
            picture_hashes,
            bio: onboarding.bio,
            birth_date: onboarding.birth_date,
            gender: onboarding.gender,
            sexual_orientation: onboarding.sexual_orientation,
            min_age: onboarding.min_age,
            max_age: onboarding.max_age,
            max_distance_km: onboarding.max_distance_km,
            location,
        })
        .await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "users",
    operation_id = "get_my_channels",
    summary = "Get the current user channels",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(session, chat_service, profile_service))]
pub async fn get_my_channels(
    chat_service: web::Data<Arc<dyn ChatService>>,
    profile_service: web::Data<Arc<dyn UserProfileService>>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<web::Json<Vec<ChannelDto>>, ApiError> {
    let user = session.authenticated_user()?;
    let profile = profile_service.get_by_user_id(user.id).await?; // TODO: need to find a way to avoid always querying the user profile

    let channels = chat_service
        .get_user_channels(profile.id, &Default::default()) // TODO: add query params
        .await?;

    let participant_futures = channels
        .iter()
        .map(|channel| chat_service.get_channel_participants(channel.id));

    let participants_results: Vec<Result<Vec<ChannelParticipant>, _>> = join_all(participant_futures).await;

    let mut channels_dto = Vec::new();

    for (channel, participants_result) in channels.into_iter().zip(participants_results) {
        let participants = participants_result.unwrap_or_else(|_| vec![]);

        let mut channel_dto: ChannelDto = channel.into();
        channel_dto.append_participants(participants);

        channels_dto.push(channel_dto);
    }

    Ok(web::Json(channels_dto))
}
