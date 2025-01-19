use std::sync::Arc;

use actix_multipart::form::MultipartForm;
use actix_web::web;
use apistos::actix::NoContent;
use apistos::api_operation;
use garde::{Error, Path, Report, Validate};
use geo_types::Point;

use crate::domain::constants::PROFILE_IMAGES_PATH;
use crate::domain::errors::user_profile_error::UserProfileError;
use crate::domain::services::cdn_service::CdnService;
use crate::domain::services::profile_tag_service::ProfileTagService;
use crate::domain::services::user_profile_service::UserProfileService;
use crate::domain::services::user_service::UserService;
use crate::infrastructure::error::ApiError;
use crate::infrastructure::models::user_profile::{UserProfileInsert, UserProfileUpdate};
use crate::infrastructure::services::iploc::locate_ip;
use crate::presentation::dto::user_dto::{UpdateUserDto, UserDto};
use crate::presentation::dto::user_profile::{
    CompleteOnboardingForm, UpdateProfileDto, UserProfileBulkTagsDto, UserProfileDto, UserProfileQueryParamsDto,
    UserProfileTagParamsDto,
};
use crate::presentation::extractors::auth_extractor::Session;
use crate::shared::types::peer_infos::PeerInfos;
use crate::shared::types::snowflake::Snowflake;
use crate::trace_peer_infos;

#[api_operation(
    tag = "users",
    operation_id = "get_me",
    summary = "Get the current user",
    skip_args = "peer_infos"
)]
pub async fn get_me(session: Session, peer_infos: PeerInfos) -> Result<web::Json<UserDto>, ApiError> {
    trace_peer_infos!(peer_infos);

    let user = session.authenticated_user()?;

    Ok(web::Json(user.clone().into()))
}

#[api_operation(
    tag = "users",
    operation_id = "update_me",
    summary = "Update the current user",
    skip_args = "peer_infos"
)]
pub async fn update_me(
    user_service: web::Data<Arc<dyn UserService>>,
    body: web::Json<UpdateUserDto>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    trace_peer_infos!(peer_infos);

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
pub async fn complete_onboarding(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    cdn_service: web::Data<Arc<dyn CdnService>>,
    MultipartForm(form): MultipartForm<CompleteOnboardingForm>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    trace_peer_infos!(peer_infos);

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
            age: onboarding.age,
            gender: onboarding.gender,
            sexual_orientation: onboarding.sexual_orientation,
            location,
        })
        .await?;

    Ok(NoContent)
}
