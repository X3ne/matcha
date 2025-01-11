use crate::presentation::controllers::user_controller::get_me;
use apistos::web;
use apistos::web::{resource, scope};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/users")
            // .service(
            //     scope("/{user_id}")
            //             .service(resource("").route(web::get().to(login_42)))
            //             .service(resource("").route(web::get().to(callback_42))),
            // )
            .service(resource("/@me").route(web::post().to(get_me))),
    );
}
