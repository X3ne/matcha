use apistos::web;
use apistos::web::{resource, scope};

use crate::presentation::controllers::gateway_controller::connect_to_gateway;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/gateway").service(resource("").route(web::get().to(connect_to_gateway))));
}
