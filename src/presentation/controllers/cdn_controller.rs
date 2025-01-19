use actix_web::{web, HttpResponse};
use apistos::api_operation;
use std::sync::Arc;

use crate::domain::constants::PROFILE_IMAGES_PATH;
use crate::domain::services::cdn_service::CdnService;
use crate::infrastructure::error::ApiError;
use crate::presentation::extractors::auth_extractor::Session;
use crate::shared::types::peer_infos::PeerInfos;
use crate::trace_peer_infos;

#[api_operation(
    tag = "cdn",
    operation_id = "get_profile_image",
    summary = "Get a profile image",
    skip_args = "peer_infos"
)]
pub async fn get_profile_image(
    cdn_service: web::Data<Arc<dyn CdnService>>,
    hash: web::Path<String>,
    _: Session,
    peer_infos: PeerInfos,
) -> Result<HttpResponse, ApiError> {
    trace_peer_infos!(peer_infos);

    let hash = hash.into_inner();
    let image = cdn_service
        .get_by_hash(&format!("{}/{}", PROFILE_IMAGES_PATH, hash))
        .await?;

    Ok(HttpResponse::Ok().content_type("image/webp".to_string()).body(image))
}
