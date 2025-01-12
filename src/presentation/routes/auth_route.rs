use apistos::web;
use apistos::web::{resource, scope};

use crate::presentation::controllers::auth_controller::{
    activate_account, callback_42, login, login_42, logout, register,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/auth")
            .service(
                scope("/oauth2").service(
                    scope("/42")
                        .service(resource("/login").route(web::get().to(login_42)))
                        .service(resource("/callback").route(web::get().to(callback_42))),
                ),
            )
            .service(
                scope("")
                    .service(resource("/register").route(web::post().to(register)))
                    .service(resource("/login").route(web::post().to(login)))
                    .service(resource("/activate").route(web::get().to(activate_account)))
                    .service(resource("/logout").route(web::post().to(logout))),
            ),
    );
}
