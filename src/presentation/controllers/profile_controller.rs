use std::sync::Arc;

use actix_web::web;
use apistos::actix::NoContent;
use apistos::api_operation;
use garde::{Error, Path, Report, Validate};

use crate::domain::services::profile_tag_service::ProfileTagService;
use crate::domain::services::user_profile_service::UserProfileService;
use crate::infrastructure::error::ApiError;
use crate::infrastructure::models::user_profile::UserProfileUpdate;
use crate::presentation::dto::user_profile::{
    UpdateProfileDto, UserProfileBulkTagsDto, UserProfileDto, UserProfileQueryParamsDto, UserProfileTagParamsDto,
};
use crate::presentation::extractors::auth_extractor::Session;
use crate::shared::types::peer_infos::PeerInfos;
use crate::shared::types::snowflake::Snowflake;

#[api_operation(
    tag = "profiles",
    operation_id = "get_my_profile",
    summary = "Get the current user profile",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, session))]
pub async fn get_my_profile(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<web::Json<UserProfileDto>, ApiError> {
    let user = session.authenticated_user()?;

    let profile = user_profile_service.get_by_user_id(user.id).await?;
    let tags = user_profile_service.get_profile_tags(profile.id).await?;

    let mut profile_dto: UserProfileDto = profile.into();
    profile_dto.append_tags(tags);

    Ok(web::Json(profile_dto))
}

#[api_operation(
    tag = "profiles",
    operation_id = "update_my_profile",
    summary = "Update the current user profile",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, session))]
pub async fn update_my_profile(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    body: web::Json<UpdateProfileDto>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;

    let body = body.into_inner();
    body.validate()?;

    let profile = user_profile_service.get_by_user_id(user.id).await?;

    if let Some(avatar_index) = body.avatar_index {
        if avatar_index >= profile.picture_hashes.len() {
            // can't pass garde context to validate this (https://github.com/jprochazk/garde/issues/104)
            let mut report = Report::new();
            report.append(Path::new("avatar_index"), Error::new("Invalid avatar index"));
            return Err(ApiError::ValidationError(report));
        }
    }

    let avatar_hash = match body.avatar_index {
        Some(index) => profile.picture_hashes.get(index).cloned(),
        None => None,
    };

    user_profile_service
        .update(
            profile.id,
            &UserProfileUpdate {
                name: body.name,
                avatar_hash,
                bio: body.bio,
                age: body.age,
                gender: body.gender,
                sexual_orientation: body.sexual_orientation,
                location: body.location.map(Into::into),
                rating: None,
            },
        )
        .await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "profiles",
    operation_id = "get_user_profile_by_id",
    summary = "Get the user profile by id",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, session))]
pub async fn get_user_profile_by_id(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    profile_id: web::Path<Snowflake>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<web::Json<UserProfileDto>, ApiError> {
    let profile_id = profile_id.into_inner();

    let profile = user_profile_service.get_by_id(profile_id).await?;

    Ok(web::Json(profile.into()))
}

#[api_operation(
    tag = "profiles",
    operation_id = "search_profile",
    summary = "Search user profiles",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, session))]
pub async fn search_profiles(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    params: web::Query<UserProfileQueryParamsDto>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<web::Json<Vec<UserProfileDto>>, ApiError> {
    let params = params.into_inner();
    params.validate()?;

    let profiles = user_profile_service.search(params.into()).await?;

    Ok(web::Json(profiles.into_iter().map(|p| p.into()).collect()))
}

#[api_operation(
    tag = "profiles",
    operation_id = "add_tag_to_my_profile",
    summary = "Add a tag to my profile",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, profile_tag_service, session))]
pub async fn add_tag_to_my_profile(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    profile_tag_service: web::Data<Arc<dyn ProfileTagService>>,
    params: web::Query<UserProfileTagParamsDto>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;

    let params = params.into_inner();

    let profile = user_profile_service.get_by_user_id(user.id).await?;
    let tag = profile_tag_service.get_by_id(params.tag_id).await?;

    user_profile_service.add_tag(profile.id, tag.id).await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "profiles",
    operation_id = "remove_tag_from_my_profile",
    summary = "Remove a tag from my profile",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, profile_tag_service, session))]
pub async fn remove_tag_from_my_profile(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    profile_tag_service: web::Data<Arc<dyn ProfileTagService>>,
    params: web::Query<UserProfileTagParamsDto>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;

    let params = params.into_inner();

    let profile = user_profile_service.get_by_user_id(user.id).await?;
    let tag = profile_tag_service.get_by_id(params.tag_id).await?;

    user_profile_service.remove_tag(profile.id, tag.id).await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "profiles",
    operation_id = "bulk_add_tag_to_my_profile",
    summary = "Bulk add tags to my profile",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, profile_tag_service, session))]
pub async fn bulk_add_tag_to_my_profile(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    profile_tag_service: web::Data<Arc<dyn ProfileTagService>>,
    body: web::Json<UserProfileBulkTagsDto>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;

    let body = body.into_inner();

    let profile = user_profile_service.get_by_user_id(user.id).await?;
    let _ = profile_tag_service.get_by_ids(body.tag_ids.clone()).await?;

    user_profile_service.bulk_add_tags(profile.id, body.tag_ids).await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "profiles",
    operation_id = "bulk_remove_tag_from_my_profile",
    summary = "Bulk remove tags from my profile",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, profile_tag_service, session))]
pub async fn bulk_remove_tag_from_my_profile(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    profile_tag_service: web::Data<Arc<dyn ProfileTagService>>,
    body: web::Json<UserProfileBulkTagsDto>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;

    let body = body.into_inner();

    let profile = user_profile_service.get_by_user_id(user.id).await?;
    let _ = profile_tag_service.get_by_ids(body.tag_ids.clone()).await?;

    user_profile_service.bulk_remove_tags(profile.id, body.tag_ids).await?;

    Ok(NoContent)
}
