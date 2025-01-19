use apistos::web;
use apistos::web::{resource, scope};

use crate::presentation::controllers::profile_controller::{
    add_tag_to_my_profile, bulk_add_tag_to_my_profile, bulk_remove_tag_from_my_profile, get_my_profile,
    get_user_profile_by_id, remove_tag_from_my_profile, search_profiles, update_my_profile,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/profiles")
            .service(resource("/search").route(web::get().to(search_profiles)))
            .service(
                resource("/@me")
                    .route(web::get().to(get_my_profile))
                    .route(web::patch().to(update_my_profile)),
            )
            .service(
                resource("/@me/tags")
                    .route(web::put().to(add_tag_to_my_profile))
                    .route(web::delete().to(remove_tag_from_my_profile)),
            )
            .service(
                resource("/@me/tags/bulk")
                    .route(web::put().to(bulk_add_tag_to_my_profile))
                    .route(web::delete().to(bulk_remove_tag_from_my_profile)),
            )
            .service(resource("/{profile_id}").route(web::get().to(get_user_profile_by_id))),
    );
}
