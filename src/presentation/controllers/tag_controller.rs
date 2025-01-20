use actix_web::web;
use apistos::api_operation;
use std::sync::Arc;

use crate::domain::entities::profile_tag::ProfileTag;
use crate::domain::services::profile_tag_service::ProfileTagService;
use crate::infrastructure::error::ApiError;
use crate::presentation::extractors::auth_extractor::Session;
use crate::shared::types::peer_infos::PeerInfos;

#[api_operation(
    tag = "tags",
    operation_id = "get_all_tags",
    summary = "Get all profile tags",
    skip_args = "_peer_infos"
)]
#[tracing::instrument(skip(profile_tag_service, _session))]
pub async fn get_all_tags(
    profile_tag_service: web::Data<Arc<dyn ProfileTagService>>,
    _session: Session,
    _peer_infos: PeerInfos,
) -> Result<web::Json<Vec<ProfileTag>>, ApiError> {
    let tags = profile_tag_service.get_all().await?;

    Ok(web::Json(tags))
}
