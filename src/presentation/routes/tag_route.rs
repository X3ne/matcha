use apistos::web;
use apistos::web::{resource, scope};

use crate::presentation::controllers::tag_controller::get_all_tags;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/tags").service(resource("").route(web::get().to(get_all_tags))));
}
