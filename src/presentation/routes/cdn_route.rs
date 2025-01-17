use apistos::web;
use apistos::web::{resource, scope};

use crate::presentation::controllers::cdn_controller::get_profile_image;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/cdn").service(scope("/profile").service(resource("/{hash}").route(web::get().to(get_profile_image)))),
    );
}
