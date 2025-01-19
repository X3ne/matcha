use std::sync::Arc;

use actix_web::{web, HttpResponse};
use apistos::actix::NoContent;
use apistos::api_operation;
use garde::Validate;
use oauth2::client::providers::ProviderKind;

use crate::config::Config;
use crate::domain::services::auth_service::AuthService;
use crate::infrastructure::error::ApiError;
use crate::presentation::dto::auth_dto::{
    ActivateAccountDto, LoginDto, OAuthCallbackQueryDto, OAuthResponseDto, RegisterUserDto,
};
use crate::presentation::dto::user_dto::ResetPasswordDto;
use crate::presentation::extractors::auth_extractor::Session;
use crate::shared::types::peer_infos::PeerInfos;
use crate::shared::utils::validation::ValidatePasswordContext;

#[api_operation(
    tag = "auth",
    operation_id = "register",
    summary = "Register a new user",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(auth_service))]
pub async fn register(
    auth_service: web::Data<Arc<dyn AuthService>>,
    body: web::Json<RegisterUserDto>,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
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
#[tracing::instrument(skip(auth_service, session))]
pub async fn login(
    auth_service: web::Data<Arc<dyn AuthService>>,
    body: web::Json<LoginDto>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
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
#[tracing::instrument(skip(auth_service))]
pub async fn login_42(
    auth_service: web::Data<Arc<dyn AuthService>>,
    peer_infos: PeerInfos,
) -> Result<web::Json<OAuthResponseDto>, ApiError> {
    let (auth_url, _csrf_state) = auth_service.generate_oauth_url(ProviderKind::Ft).await?;

    Ok(web::Json(OAuthResponseDto {
        url: auth_url.to_string(),
    }))
}

#[api_operation(tag = "auth", operation_id = "callback_42", summary = "Callback for 42 OAuth", skip_args = peer_infos)]
#[tracing::instrument(skip(auth_service, cfg, session))]
pub async fn callback_42(
    query: web::Query<OAuthCallbackQueryDto>,
    auth_service: web::Data<Arc<dyn AuthService>>,
    cfg: web::Data<Arc<Config>>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<HttpResponse, ApiError> {
    let OAuthCallbackQueryDto { code, state } = query.into_inner();

    let user = auth_service.oauth_callback(ProviderKind::Ft, code, state).await?;

    let _ = session.inner().insert("user_id", user.id.to_string());
    session.inner().renew();

    match cfg.client_base_url.as_ref() {
        Some(uri) => Ok(HttpResponse::Found().append_header(("Location", uri.clone())).finish()),
        None => Ok(HttpResponse::Ok().finish()),
    }
}

#[api_operation(
    tag = "auth",
    operation_id = "activate_account",
    summary = "Activate the user account",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(auth_service))]
pub async fn activate_account(
    auth_service: web::Data<Arc<dyn AuthService>>,
    query: web::Query<ActivateAccountDto>,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let token = query.into_inner().token;

    auth_service.activate_account(token).await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "auth",
    operation_id = "request_reset_password",
    summary = "Request a password reset",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(auth_service, cfg, session))]
pub async fn request_reset_password(
    auth_service: web::Data<Arc<dyn AuthService>>,
    cfg: web::Data<Arc<Config>>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;

    auth_service
        .request_password_reset(&user.email, &cfg.reset_password_url)
        .await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "auth",
    operation_id = "reset_password",
    summary = "Reset the user password",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(auth_service, session))]
pub async fn reset_password(
    auth_service: web::Data<Arc<dyn AuthService>>,
    body: web::Json<ResetPasswordDto>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let user = session.authenticated_user()?;

    let body = body.into_inner();
    body.validate_with(&ValidatePasswordContext {
        username: user.username.clone(),
        last_name: user.last_name.clone(),
        first_name: user.first_name.clone(),
        email: user.email.clone(),
    })?;

    auth_service.reset_password(&body.token, &body.password).await?;

    Ok(NoContent)
}

#[api_operation(
    tag = "auth",
    operation_id = "logout",
    summary = "Logout the current user",
    skip_args = "peer_infos"
)]
#[tracing::instrument(skip(session))]
pub async fn logout(session: Session, peer_infos: PeerInfos) -> Result<NoContent, ApiError> {
    session.inner().clear();

    Ok(NoContent)
}
