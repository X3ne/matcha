use apistos::web;
use apistos::web::{resource, scope};

use crate::presentation::controllers::chat_controller::{
    get_channel_message, get_channel_messages, post_channel_message,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/channels").service(
            scope("/{channel_id}")
                .service(
                    resource("/messages")
                        .route(web::get().to(get_channel_messages))
                        .route(web::post().to(post_channel_message)),
                )
                .service(resource("/{message_id}").route(web::get().to(get_channel_message))),
        ),
    );
}
