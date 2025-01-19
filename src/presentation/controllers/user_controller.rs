use actix_multipart::form::MultipartForm;
use actix_web::web;
use apistos::actix::NoContent;
use apistos::api_operation;
use garde::{Error, Path, Report, Validate};
use geo_types::Point;
use std::sync::Arc;

use crate::domain::constants::PROFILE_IMAGES_PATH;
use crate::domain::errors::user_profile_error::UserProfileError;
use crate::domain::services::cdn_service::CdnService;
use crate::domain::services::profile_tag_service::ProfileTagService;
use crate::domain::services::user_profile_service::UserProfileService;
use crate::domain::services::user_service::UserService;
use crate::infrastructure::error::ApiError;
use crate::infrastructure::models::user_profile::{UserProfileInsert, UserProfileUpdate};
use crate::presentation::dto::user_dto::{UpdateUserDto, UserDto};
use crate::presentation::dto::user_profile::{
    CompleteOnboardingForm, UpdateProfileDto, UserProfileBulkTagsDto, UserProfileDto, UserProfileQueryParamsDto,
    UserProfileTagParamsDto,
};
use crate::presentation::extractors::auth_extractor::Session;
use crate::shared::types::snowflake::Snowflake;

#[api_operation(tag = "users", operation_id = "get_me", summary = "Get the current user")]
pub async fn get_me(session: Session) -> Result<web::Json<UserDto>, ApiError> {
    let user = session.authenticated_user()?;

    Ok(web::Json(user.clone().into()))
}

#[api_operation(tag = "users", operation_id = "update_me", summary = "Update the current user")]
pub async fn update_me(
    user_service: web::Data<Arc<dyn UserService>>,
    body: web::Json<UpdateUserDto>,
    session: Session,
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
    summary = "Complete the onboarding process"
)]
pub async fn complete_onboarding(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    cdn_service: web::Data<Arc<dyn CdnService>>,
    session: Session,
    MultipartForm(form): MultipartForm<CompleteOnboardingForm>,
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
        None => Point::new(0.0, 0.0), // TODO: use ip to get location
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

#[api_operation(
    tag = "users",
    operation_id = "get_my_profile",
    summary = "Get the current user profile"
)]
pub async fn get_my_profile(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    session: Session,
) -> Result<web::Json<UserProfileDto>, ApiError> {
    let user = session.authenticated_user()?;

    let profile = user_profile_service.get_by_user_id(user.id).await?;
    let tags = user_profile_service.get_profile_tags(profile.id).await?;

    let mut profile_dto: UserProfileDto = profile.into();
    profile_dto.append_tags(tags);

    Ok(web::Json(profile_dto))
}

#[api_operation(
    tag = "users",
    operation_id = "update_my_profile",
    summary = "Update the current user profile"
)]
pub async fn update_my_profile(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    body: web::Json<UpdateProfileDto>,
    session: Session,
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
    tag = "users",
    operation_id = "get_user_profile_by_id",
    summary = "Get the user profile by id"
)]
pub async fn get_user_profile_by_id(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    profile_id: web::Path<Snowflake>,
    _: Session,
) -> Result<web::Json<UserProfileDto>, ApiError> {
    let profile_id = profile_id.into_inner();

    let profile = user_profile_service.get_by_id(profile_id).await?;

    Ok(web::Json(profile.into()))
}

#[api_operation(tag = "users", operation_id = "search_profile", summary = "Search user profiles")]
pub async fn search_profiles(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    params: web::Query<UserProfileQueryParamsDto>,
    _: Session,
) -> Result<web::Json<Vec<UserProfileDto>>, ApiError> {
    let params = params.into_inner();
    params.validate()?;

    let profiles = user_profile_service.search(params.into()).await?;

    Ok(web::Json(profiles.into_iter().map(|p| p.into()).collect()))
}

#[api_operation(
    tag = "users",
    operation_id = "add_tag_to_my_profile",
    summary = "Add a tag to my profile"
)]
pub async fn add_tag_to_my_profile(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    profile_tag_service: web::Data<Arc<dyn ProfileTagService>>,
    params: web::Query<UserProfileTagParamsDto>,
    session: Session,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;

    let params = params.into_inner();

    let profile = user_profile_service.get_by_user_id(user.id).await?;
    let tag = profile_tag_service.get_by_id(params.tag_id).await?;

    user_profile_service.add_tag(profile.id, tag.id).await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "users",
    operation_id = "remove_tag_from_my_profile",
    summary = "Remove a tag from my profile"
)]
pub async fn remove_tag_from_my_profile(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    profile_tag_service: web::Data<Arc<dyn ProfileTagService>>,
    params: web::Query<UserProfileTagParamsDto>,
    session: Session,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;

    let params = params.into_inner();

    let profile = user_profile_service.get_by_user_id(user.id).await?;
    let tag = profile_tag_service.get_by_id(params.tag_id).await?;

    user_profile_service.remove_tag(profile.id, tag.id).await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "users",
    operation_id = "bulk_add_tag_to_my_profile",
    summary = "Bulk add tags to my profile"
)]
pub async fn bulk_add_tag_to_my_profile(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    profile_tag_service: web::Data<Arc<dyn ProfileTagService>>,
    body: web::Json<UserProfileBulkTagsDto>,
    session: Session,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;

    let body = body.into_inner();

    let profile = user_profile_service.get_by_user_id(user.id).await?;
    let _ = profile_tag_service.get_by_ids(body.tag_ids.clone()).await?;

    user_profile_service.bulk_add_tags(profile.id, body.tag_ids).await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "users",
    operation_id = "bulk_remove_tag_from_my_profile",
    summary = "Bulk remove tags from my profile"
)]
pub async fn bulk_remove_tag_from_my_profile(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    profile_tag_service: web::Data<Arc<dyn ProfileTagService>>,
    body: web::Json<UserProfileBulkTagsDto>,
    session: Session,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;

    let body = body.into_inner();

    let profile = user_profile_service.get_by_user_id(user.id).await?;
    let _ = profile_tag_service.get_by_ids(body.tag_ids.clone()).await?;

    user_profile_service.bulk_remove_tags(profile.id, body.tag_ids).await?;

    Ok(NoContent)
}
