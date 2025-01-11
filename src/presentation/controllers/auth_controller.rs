use crate::infrastructure::error::ApiError;
use crate::presentation::dto::auth_dto::{OAuthCallbackQuery, OAuthResponse};
use std::sync::Arc;

use crate::domain::services::auth_service::AuthService;
use crate::presentation::extractors::auth_extractor::Session;
use crate::shared::types::peer_infos::PeerInfos;
use actix_web::web;
use apistos::actix::NoContent;
use apistos::api_operation;
use oauth2::client::providers::ProviderKind;
// **
// * TODO:
// * - Implement pkce
// * - Implement csrf
// * - make oauth routes generics
// **

#[api_operation(tag = "auth", operation_id = "login_42", summary = "Login with 42 account")]
pub async fn login_42(auth_service: web::Data<Arc<dyn AuthService>>) -> Result<web::Json<OAuthResponse>, ApiError> {
    let (auth_url, _csrf_state) = auth_service.generate_oauth_url(ProviderKind::Ft).await?;

    Ok(web::Json(OAuthResponse {
        url: auth_url.to_string(),
    }))
}

#[api_operation(tag = "auth", operation_id = "callback_42", summary = "Callback for 42 OAuth", skip_args = peer_infos)]
pub async fn callback_42(
    query: web::Query<OAuthCallbackQuery>,
    auth_service: web::Data<Arc<dyn AuthService>>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<NoContent, ApiError> {
    let OAuthCallbackQuery { code, state } = query.into_inner();

    let user = auth_service.oauth_callback(ProviderKind::Ft, code, state).await?;

    let _ = session.inner().insert("user_id", user.id.to_string());
    session.inner().renew();

    Ok(NoContent)
}

#[api_operation(
    tag = "auth",
    operation_id = "logout",
    summary = "Logout the current user",
    description = "This endpoint can use the `session` cookie to logout the user"
)]
pub async fn logout() -> Result<NoContent, ApiError> {
    Ok(NoContent)
}
