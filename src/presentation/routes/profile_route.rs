use apistos::web;
use apistos::web::{resource, scope};

use crate::presentation::controllers::profile_controller::{
    add_tag_to_my_profile, bulk_add_tag_to_my_profile, bulk_remove_tag_from_my_profile, delete_profile_picture,
    get_my_profile, get_user_profile_by_id, remove_tag_from_my_profile, search_profiles, set_default_profile_picture,
    update_my_profile, upload_profile_picture,
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
                scope("/@me/pictures")
                    .service(resource("").route(web::post().to(upload_profile_picture)))
                    .service(resource("/{picture_offset}").route(web::delete().to(delete_profile_picture)))
                    .service(resource("/{picture_offset}/default").route(web::put().to(set_default_profile_picture))),
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
