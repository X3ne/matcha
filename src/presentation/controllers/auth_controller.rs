use std::sync::Arc;

use actix_web::web;
use apistos::actix::NoContent;
use apistos::api_operation;
use oauth2::client::providers::ProviderKind;

use crate::domain::services::auth_service::AuthService;
use crate::infrastructure::error::ApiError;
use crate::infrastructure::models::user::UserInsert;
use crate::presentation::dto::auth_dto::{
    ActivateAccountDto, LoginDto, OAuthCallbackQueryDto, OAuthResponseDto, RegisterUserDto,
};
use crate::presentation::extractors::auth_extractor::Session;
use crate::shared::types::peer_infos::PeerInfos;
// **
// * TODO:
// * - Implement pkce
// * - Implement csrf
// * - make oauth routes generics
// **

#[api_operation(tag = "auth", operation_id = "register", summary = "Register a new user")]
pub async fn register(
    auth_service: web::Data<Arc<dyn AuthService>>,
    body: web::Json<RegisterUserDto>,
) -> Result<NoContent, ApiError> {
    let mut user: UserInsert = body.into_inner().into();

    auth_service.register(&mut user).await?;

    Ok(NoContent)
}

#[api_operation(tag = "auth", operation_id = "login", summary = "Login with credentials")]
pub async fn login(
    auth_service: web::Data<Arc<dyn AuthService>>,
    body: web::Json<LoginDto>,
    session: Session,
) -> Result<NoContent, ApiError> {
    let LoginDto { username, password } = body.into_inner();

    let user = auth_service.login(&username, &password).await?;

    let _ = session.inner().insert("user_id", user.id.to_string());
    session.inner().renew();

    Ok(NoContent)
}

#[api_operation(tag = "auth", operation_id = "login_42", summary = "Login with 42 account")]
pub async fn login_42(auth_service: web::Data<Arc<dyn AuthService>>) -> Result<web::Json<OAuthResponseDto>, ApiError> {
    let (auth_url, _csrf_state) = auth_service.generate_oauth_url(ProviderKind::Ft).await?;

    Ok(web::Json(OAuthResponseDto {
        url: auth_url.to_string(),
    }))
}

#[api_operation(tag = "auth", operation_id = "callback_42", summary = "Callback for 42 OAuth", skip_args = peer_infos)]
pub async fn callback_42(
    query: web::Query<OAuthCallbackQueryDto>,
    auth_service: web::Data<Arc<dyn AuthService>>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let OAuthCallbackQueryDto { code, state } = query.into_inner();

    let user = auth_service.oauth_callback(ProviderKind::Ft, code, state).await?;

    let _ = session.inner().insert("user_id", user.id.to_string());
    session.inner().renew();

    Ok(NoContent)
}

#[api_operation(
    tag = "auth",
    operation_id = "activate_account",
    summary = "Activate the user account"
)]
pub async fn activate_account(
    auth_service: web::Data<Arc<dyn AuthService>>,
    query: web::Query<ActivateAccountDto>, // TODO: add redirect_url to redirect the user after activation
) -> Result<NoContent, ApiError> {
    let token = query.into_inner().token;

    auth_service.activate_account(token).await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "auth",
    operation_id = "logout",
    summary = "Logout the current user",
    description = "This endpoint can use the `session` cookie to logout the user"
)]
pub async fn logout(session: Session) -> Result<NoContent, ApiError> {
    session.inner().clear();

    Ok(NoContent)
}
