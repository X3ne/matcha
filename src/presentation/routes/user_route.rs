use apistos::web;
use apistos::web::{resource, scope};

use crate::presentation::controllers::user_controller::{complete_onboarding, get_me, update_me};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/users")
            .service(
                resource("/@me")
                    .route(web::get().to(get_me))
                    .route(web::patch().to(update_me)),
            )
            .service(resource("/@me/onboarding").route(web::post().to(complete_onboarding))),
    );
}
