use apistos::web;
use apistos::web::{resource, scope};

use crate::presentation::controllers::user_controller::{complete_onboarding, get_me, get_my_profile};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/users")
            // .service(
            //     scope("/{user_id}")
            //             .service(resource("").route(web::get().to(login_42)))
            //             .service(resource("").route(web::get().to(callback_42))),
            // )
            .service(
                scope("/@me")
                    .service(resource("").route(web::get().to(get_me)))
                    .service(resource("/onboarding").route(web::post().to(complete_onboarding)))
                    .service(resource("/profile").route(web::get().to(get_my_profile))),
            ),
    );
}
