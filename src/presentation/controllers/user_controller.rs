use crate::domain::services::user_profile_service::UserProfileService;
use crate::infrastructure::error::ApiError;
use crate::infrastructure::models::user_profile::UserProfileInsert;
use crate::presentation::dto::user_dto::UserDto;
use crate::presentation::dto::user_profile::{CompleteOnboardingDto, UserProfileDto};
use crate::presentation::extractors::auth_extractor::Session;
use actix_web::web;
use apistos::actix::NoContent;
use apistos::api_operation;
use geo_types::Point;
use std::sync::Arc;

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
    session: Session,
    body: web::Json<CompleteOnboardingDto>,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;

    let onboarding = body.into_inner();

    let profile = user_profile_service
        .create(&UserProfileInsert {
            user_id: user.id,
            name: format!("{} {}", user.first_name, user.last_name),
            avatar_hash: None, // TODO: add upload avatar (this route should take a multipart form)
            bio: onboarding.bio,
            age: onboarding.age,
            gender: onboarding.gender,
            sexual_orientation: onboarding.sexual_orientation,
            location: Point::new(0.0, 0.0), // TODO: send location or use ip to get location
            tags: onboarding.tags.unwrap_or_default(),
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
