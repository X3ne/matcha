use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use futures::future::{ready, Ready};

use crate::infrastructure::error::ApiError;
use crate::shared::types::peer_infos::PeerInfos;

impl FromRequest for PeerInfos {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let infos = PeerInfos::from(req.clone());

        ready(Ok(infos))
    }
}
