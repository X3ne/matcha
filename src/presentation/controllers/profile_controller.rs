use std::sync::Arc;

use actix_multipart::form::MultipartForm;
use actix_web::web;
use apistos::actix::NoContent;
use apistos::api_operation;
use futures::future::join_all;
use garde::Validate;
use geo_types::Point;

use crate::domain::constants::PROFILE_IMAGES_PATH;
use crate::domain::entities::profile_tag::ProfileTag;
use crate::domain::errors::user_profile_error::UserProfileError;
use crate::domain::repositories::user_profile_repo::UserProfileQueryParams;
use crate::domain::services::cdn_service::CdnService;
use crate::domain::services::profile_tag_service::ProfileTagService;
use crate::domain::services::user_profile_service::UserProfileService;
use crate::infrastructure::error::ApiError;
use crate::infrastructure::models::user_profile::UserProfileUpdate;
use crate::presentation::dto::user_profile::{
    UpdateProfileDto, UploadProfilePictureForm, UserProfileBulkTagsDto, UserProfileDto, UserProfileMeta,
    UserProfileQueryParamsDto, UserProfileTagParamsDto,
};
use crate::presentation::extractors::auth_extractor::Session;
use crate::shared::types::peer_infos::PeerInfos;
use crate::shared::types::snowflake::Snowflake;
use crate::shared::utils::approx_distance_km;

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

    user_profile_service
        .update(
            profile.id,
            &UserProfileUpdate {
                name: body.name,
                bio: body.bio,
                gender: body.gender,
                sexual_orientation: body.sexual_orientation,
                location: body.location.map(Into::into),
                ..Default::default()
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
    let user = session.authenticated_user()?;
    let profile_id = profile_id.into_inner();

    let (user_profile, profile_data, tags) = tokio::try_join!(
        user_profile_service.get_by_user_id(user.id),
        user_profile_service.get_by_id(profile_id),
        user_profile_service.get_profile_tags(profile_id),
    )?;

    let (is_liked, is_a_match) = tokio::try_join!(
        user_profile_service.is_profile_liked(user_profile.id, profile_data.id),
        user_profile_service.is_profile_matched(user_profile.id, profile_data.id),
    )?;

    let approx_distance = approx_distance_km(&user_profile.location, &profile_data.location);

    let mut profile: UserProfileDto = profile_data.into();
    profile.append_tags(tags);
    profile.set_approx_distance(approx_distance);
    profile.set_meta(UserProfileMeta { is_liked, is_a_match });

    Ok(web::Json(profile))
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

    let user = session.authenticated_user()?;
    let user_profile = user_profile_service.get_by_user_id(user.id).await?;
    let user_tags = user_profile_service.get_profile_tags(user_profile.id).await?;

    let mut search_params: UserProfileQueryParams = params.into();
    if search_params.location.is_none() {
        let point: Point<f64> = user_profile
            .location
            .clone()
            .try_into()
            .map_err(|_| ApiError::InternalServerError)?;
        search_params.location = Some(point);
    }

    if search_params.tag_ids.is_none() {
        search_params.tag_ids = Some(user_tags.iter().map(|tag| tag.id).collect());
    }

    let profiles = user_profile_service.search(&search_params).await?;

    let tag_futures: Vec<_> = profiles
        .iter()
        .map(|profile| user_profile_service.get_profile_tags(profile.id))
        .collect();

    let tags_results: Vec<Result<Vec<ProfileTag>, _>> = join_all(tag_futures).await;

    let mut profiles_dto = Vec::new();

    for (profile, tags_result) in profiles.into_iter().zip(tags_results) {
        let tags = tags_result.unwrap_or_else(|_| vec![]);
        let approx_distance = approx_distance_km(&user_profile.location, &profile.location);

        let (is_liked, is_a_match) = tokio::try_join!(
            user_profile_service.is_profile_liked(user_profile.id, profile.id),
            user_profile_service.is_profile_matched(user_profile.id, profile.id),
        )?;

        let mut profile_dto: UserProfileDto = profile.into();
        profile_dto.append_tags(tags);
        profile_dto.set_approx_distance(approx_distance);
        profile_dto.set_meta(UserProfileMeta { is_liked, is_a_match });

        profiles_dto.push(profile_dto);
    }

    Ok(web::Json(profiles_dto))
}

#[api_operation(
    tag = "profiles",
    operation_id = "recommend_profiles",
    summary = "Recommend user profiles",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, session))]
pub async fn recommend_profiles(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<web::Json<Vec<UserProfileDto>>, ApiError> {
    let user = session.authenticated_user()?;

    let user_profile = user_profile_service.get_by_user_id(user.id).await?;
    let recommendations = user_profile_service
        .recommend(
            user_profile.id,
            user_profile.location.clone(),
            50.0, // TODO: for now this is hardcoded, but it should be a user setting
            user_profile.gender,
            user_profile.sexual_orientation,
            18,
            99,
        )
        .await?;

    let tag_futures: Vec<_> = recommendations
        .iter()
        .map(|profile| user_profile_service.get_profile_tags(profile.id))
        .collect();

    let tags_results: Vec<Result<Vec<ProfileTag>, _>> = join_all(tag_futures).await;

    let mut profiles_dto = Vec::new();

    for (profile, tags_result) in recommendations.into_iter().zip(tags_results) {
        let tags = tags_result.unwrap_or_else(|_| vec![]);
        let approx_distance = approx_distance_km(&user_profile.location, &profile.location);

        let (is_liked, is_a_match) = tokio::try_join!(
            user_profile_service.is_profile_liked(user_profile.id, profile.id),
            user_profile_service.is_profile_matched(user_profile.id, profile.id),
        )?;

        let mut profile_dto: UserProfileDto = profile.into();
        profile_dto.append_tags(tags);
        profile_dto.set_approx_distance(approx_distance);
        profile_dto.set_meta(UserProfileMeta { is_liked, is_a_match });

        profiles_dto.push(profile_dto);
    }

    Ok(web::Json(profiles_dto))
}

#[api_operation(
    tag = "profiles",
    operation_id = "upload_profile_picture",
    summary = "Upload a picture to my profile",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, cdn_service, session))]
pub async fn upload_profile_picture(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    cdn_service: web::Data<Arc<dyn CdnService>>,
    MultipartForm(mut form): MultipartForm<UploadProfilePictureForm>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;
    let profile = user_profile_service.get_by_user_id(user.id).await?;

    if profile.picture_hashes.len() + 1 > 5 {
        return Err(UserProfileError::MaxImages.into());
    }

    let content_type = form
        .picture
        .content_type
        .clone()
        .ok_or(ApiError::BadRequest("Missing content type".to_string()))?;

    if content_type.type_() != mime::IMAGE {
        return Err(ApiError::OnlyImagesAllowed);
    }

    let hash = cdn_service.upload_file(&mut form.picture, PROFILE_IMAGES_PATH).await?;

    user_profile_service.add_pictures(profile.id, vec![hash]).await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "profiles",
    operation_id = "delete_profile_picture",
    summary = "Delete a picture from my profile",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, cdn_service, session))]
pub async fn delete_profile_picture(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    cdn_service: web::Data<Arc<dyn CdnService>>,
    picture_offset: web::Path<usize>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;
    let profile = user_profile_service.get_by_user_id(user.id).await?;

    let picture_offset = picture_offset.into_inner();

    let picture_hash = profile
        .picture_hashes
        .get(picture_offset)
        .cloned()
        .ok_or(UserProfileError::InvalidImageOffset)?;

    if profile.avatar_hash == Some(picture_hash.clone()) {
        return Err(UserProfileError::CannotDeleteAvatar.into());
    }

    user_profile_service
        .remove_pictures(profile.id, vec![picture_hash.clone()])
        .await?;

    let is_used = cdn_service.is_picture_hash_used(&picture_hash).await?;

    if !is_used {
        cdn_service
            .delete_file(&format!("{}/{}", PROFILE_IMAGES_PATH, picture_hash))
            .await?;
    }

    Ok(NoContent)
}

#[api_operation(
    tag = "profiles",
    operation_id = "set_default_profile_picture",
    summary = "Set a picture as default profile picture",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, session))]
pub async fn set_default_profile_picture(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    picture_offset: web::Path<usize>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;
    let profile = user_profile_service.get_by_user_id(user.id).await?;

    let picture_offset = picture_offset.into_inner();

    let picture_hash = profile
        .picture_hashes
        .get(picture_offset)
        .cloned()
        .ok_or(UserProfileError::InvalidImageOffset)?;

    user_profile_service
        .update(
            profile.id,
            &UserProfileUpdate {
                avatar_hash: Some(picture_hash),
                ..Default::default()
            },
        )
        .await?;

    Ok(NoContent)
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
    let params = params.into_inner();
    params.validate()?;

    let user = session.authenticated_user()?;

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
    let params = params.into_inner();
    params.validate()?;

    let user = session.authenticated_user()?;

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
    let body = body.into_inner();
    body.validate()?;

    let user = session.authenticated_user()?;

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
    let body = body.into_inner();
    body.validate()?;

    let user = session.authenticated_user()?;

    let profile = user_profile_service.get_by_user_id(user.id).await?;
    let _ = profile_tag_service.get_by_ids(body.tag_ids.clone()).await?;

    user_profile_service.bulk_remove_tags(profile.id, body.tag_ids).await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "profiles",
    operation_id = "get_my_profile_likes",
    summary = "Get the current user profile likes",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, session))]
pub async fn get_my_profile_likes(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<web::Json<Vec<UserProfileDto>>, ApiError> {
    let user = session.authenticated_user()?;

    let profile = user_profile_service.get_by_user_id(user.id).await?;
    let profiles = user_profile_service.get_profile_likes(profile.id).await?;

    Ok(web::Json(profiles.into_iter().map(Into::into).collect()))
}

#[api_operation(
    tag = "profiles",
    operation_id = "like_user_profile",
    summary = "Like a user profile",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, session))]
pub async fn like_user_profile(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    profile_id: web::Path<Snowflake>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;

    let profile_id = profile_id.into_inner();

    let profile = user_profile_service.get_by_user_id(user.id).await?;
    let _ = user_profile_service.add_like(&profile, profile_id).await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "profiles",
    operation_id = "remove_user_profile_like",
    summary = "Remove a like from a user profile",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, session))]
pub async fn remove_user_profile_like(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    profile_id: web::Path<Snowflake>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;

    let profile_id = profile_id.into_inner();

    let profile = user_profile_service.get_by_user_id(user.id).await?;
    let _ = user_profile_service.remove_like(profile.id, profile_id).await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "profiles",
    operation_id = "get_my_profile_matches",
    summary = "Get the current user profile matches",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(user_profile_service, session))]
pub async fn get_my_profile_matches(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<web::Json<Vec<UserProfileDto>>, ApiError> {
    let user = session.authenticated_user()?;

    let profile = user_profile_service.get_by_user_id(user.id).await?;
    let profiles = user_profile_service.get_matches(profile.id).await?;

    Ok(web::Json(profiles.into_iter().map(Into::into).collect()))
}
