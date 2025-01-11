use crate::infrastructure::error::ApiError;
use crate::presentation::dto::user_dto::UserDto;
use crate::presentation::extractors::auth_extractor::Session;
use actix_web::web;
use apistos::api_operation;

#[api_operation(tag = "users", operation_id = "get_me", summary = "Get the current user")]
pub async fn get_me(session: Session) -> Result<web::Json<UserDto>, ApiError> {
    let user = session.authenticated_user()?;

    Ok(web::Json(user.clone().into()))
}
