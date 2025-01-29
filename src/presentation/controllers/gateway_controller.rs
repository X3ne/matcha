use std::sync::Arc;

use actix_web::{web, HttpResponse};
use apistos::api_operation;

use crate::domain::services::user_profile_service::UserProfileService;
use crate::infrastructure::error::ApiError;
use crate::infrastructure::gateway::Gateway;
use crate::presentation::extractors::auth_extractor::Session;
use crate::shared::types::peer_infos::PeerInfos;

#[api_operation(
    tag = "gateway",
    operation_id = "connect_to_gateway",
    summary = "Connect to the events gateway",
    skip_args = "peer_infos"
)]
pub async fn connect_to_gateway(
    user_profile_service: web::Data<Arc<dyn UserProfileService>>,
    gateway: web::Data<Arc<Gateway>>,
    session: Session,
    peer_infos: PeerInfos,
) -> Result<HttpResponse, ApiError> {
    let user_id = session.authenticated_user()?.id;
    let profile = user_profile_service.get_by_user_id(user_id).await?;

    let res = HttpResponse::Ok()
        .insert_header(("Content-Type", "text/event-stream"))
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(gateway.register_client(profile.id).await);

    Ok(res)
}
