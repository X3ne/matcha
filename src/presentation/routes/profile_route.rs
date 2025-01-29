use apistos::web;
use apistos::web::{resource, scope};

use crate::presentation::controllers::profile_controller::{
    add_tag_to_my_profile, bulk_add_tag_to_my_profile, bulk_remove_tag_from_my_profile, delete_profile_picture,
    dislike_user_profile, get_my_profile, get_my_profile_likes, get_my_profile_matches, get_my_profile_views,
    get_user_profile_by_id, like_user_profile, recommend_profiles, remove_tag_from_my_profile,
    remove_user_profile_like, search_profiles, set_default_profile_picture, update_my_profile, upload_profile_picture,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/profiles")
            .service(resource("/search").route(web::get().to(search_profiles)))
            .service(resource("/recommend").route(web::get().to(recommend_profiles)))
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
            .service(resource("/@me/likes").route(web::get().to(get_my_profile_likes)))
            .service(resource("/@me/matches").route(web::get().to(get_my_profile_matches)))
            .service(resource("/@me/views").route(web::get().to(get_my_profile_views)))
            .service(resource("/{profile_id}").route(web::get().to(get_user_profile_by_id)))
            .service(
                resource("/{profile_id}/like")
                    .route(web::put().to(like_user_profile))
                    .route(web::delete().to(remove_user_profile_like)),
            )
            .service(resource("/{profile_id}/dislike").route(web::put().to(dislike_user_profile))),
    );
}
