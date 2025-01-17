use actix_multipart::form::MultipartForm;
use actix_web::web;
use apistos::actix::NoContent;
use apistos::api_operation;
use garde::{Error, Path, Report, Validate};
use geo_types::Point;
use std::sync::Arc;

use crate::domain::constants::PROFILE_IMAGES_PATH;
use crate::domain::errors::user_error::UserError;
use crate::domain::services::cdn_service::CdnService;
use crate::domain::services::user_profile_service::UserProfileService;
use crate::infrastructure::error::ApiError;
use crate::infrastructure::models::user_profile::UserProfileInsert;
use crate::presentation::dto::user_dto::UserDto;
use crate::presentation::dto::user_profile::{CompleteOnboardingForm, UserProfileDto, UserProfileQueryParamsDto};
use crate::presentation::extractors::auth_extractor::Session;

#[api_operation(tag = "users", operation_id = "get_me", summary = "Get the current user")]
pub async fn get_me(session: Session) -> Result<web::Json<UserDto>, ApiError> {
    let user = session.authenticated_user()?;

    Ok(web::Json(user.clone().into()))
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
        return Err(UserError::MaxImages.into());
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
            name: format!("{} {}", user.first_name, user.last_name),
            avatar_hash: picture_hashes.get(onboarding.avatar_index).cloned(),
            picture_hashes,
            bio: onboarding.bio,
            age: onboarding.age,
            gender: onboarding.gender,
            sexual_orientation: onboarding.sexual_orientation,
            location,
            tag_ids: onboarding.tag_ids,
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

    Ok(web::Json(profile.into()))
}

#[api_operation(tag = "users", operation_id = "search_profile", summary = "Search user profiles")]
pub async fn search_profiles(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    params: web::Query<UserProfileQueryParamsDto>,
    session: Session,
) -> Result<web::Json<Vec<UserProfileDto>>, ApiError> {
    // let user = session.authenticated_user()?;
    let params = params.into_inner();
    params.validate()?;

    let profiles = user_profile_service.search(params.into()).await?;

    Ok(web::Json(profiles.into_iter().map(|p| p.into()).collect()))
}
