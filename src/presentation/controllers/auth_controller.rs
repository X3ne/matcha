use std::sync::Arc;

use actix_web::web;
use apistos::actix::NoContent;
use apistos::api_operation;
use garde::Validate;
use oauth2::client::providers::ProviderKind;

use crate::domain::services::auth_service::AuthService;
use crate::infrastructure::error::ApiError;
use crate::presentation::dto::auth_dto::{
    ActivateAccountDto, LoginDto, OAuthCallbackQueryDto, OAuthResponseDto, RegisterUserDto,
};
use crate::presentation::extractors::auth_extractor::Session;
use crate::shared::types::peer_infos::PeerInfos;
use crate::shared::utils::validation::ValidatePasswordContext;
use crate::trace_peer_infos;
// **
// * TODO:
// * - Implement pkce
// * - Implement csrf
// * - make oauth routes generics
// **

#[api_operation(
    tag = "auth",
    operation_id = "register",
    summary = "Register a new user",
    skip_args = "peer_infos"
)]
pub async fn register(
    auth_service: web::Data<Arc<dyn AuthService>>,
    body: web::Json<RegisterUserDto>,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    trace_peer_infos!(peer_infos);

    let user = body.into_inner();
    user.validate_with(&ValidatePasswordContext {
        username: user.username.clone(),
        last_name: user.last_name.clone(),
        first_name: user.first_name.clone(),
        email: user.email.clone(),
    })?;

    auth_service.register(&mut user.into()).await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "auth",
    operation_id = "login",
    summary = "Login with credentials",
    skip_args = "peer_infos"
)]
pub async fn login(
    auth_service: web::Data<Arc<dyn AuthService>>,
    body: web::Json<LoginDto>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    trace_peer_infos!(peer_infos);

    let LoginDto { username, password } = body.into_inner();

    let user = auth_service.login(&username, &password).await?;

    let _ = session.inner().insert("user_id", user.id.to_string());
    session.inner().renew();

    Ok(NoContent)
}

#[api_operation(
    tag = "auth",
    operation_id = "login_42",
    summary = "Login with 42 account",
    skip_args = "peer_infos"
)]
pub async fn login_42(
    auth_service: web::Data<Arc<dyn AuthService>>,
    peer_infos: PeerInfos,
) -> Result<web::Json<OAuthResponseDto>, ApiError> {
    trace_peer_infos!(peer_infos);

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
    trace_peer_infos!(peer_infos);

    let OAuthCallbackQueryDto { code, state } = query.into_inner();

    let user = auth_service.oauth_callback(ProviderKind::Ft, code, state).await?;

    let _ = session.inner().insert("user_id", user.id.to_string());
    session.inner().renew();

    Ok(NoContent)
}

#[api_operation(
    tag = "auth",
    operation_id = "activate_account",
    summary = "Activate the user account",
    skip_args = "peer_infos"
)]
pub async fn activate_account(
    auth_service: web::Data<Arc<dyn AuthService>>,
    query: web::Query<ActivateAccountDto>, // TODO: add redirect_url to redirect the user after activation
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    trace_peer_infos!(peer_infos);

    let token = query.into_inner().token;

    auth_service.activate_account(token).await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "auth",
    operation_id = "logout",
    summary = "Logout the current user",
    description = "This endpoint can use the `session` cookie to logout the user",
    skip_args = "peer_infos"
)]
pub async fn logout(session: Session, peer_infos: PeerInfos) -> Result<NoContent, ApiError> {
    trace_peer_infos!(peer_infos);

    session.inner().clear();

    Ok(NoContent)
}
