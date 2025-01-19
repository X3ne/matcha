use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::time::Duration;
use actix_web::cookie::{Key, SameSite};
use actix_web::{web, App, HttpServer};
use apistos::app::{BuildConfig, OpenApiWrapper};
use apistos::spec::Spec;
use apistos::web::{get, resource, scope};
use apistos::{api_operation, info::Info, ApiComponent, ScalarConfig};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::task::JoinHandle;
use tracing_actix_web::TracingLogger;

use crate::infrastructure::config::Config;
use crate::infrastructure::error::ApiError;
use crate::infrastructure::web::cors::default_cors;
use crate::server::container::Container;

pub struct ActixServer {
    pub handle: JoinHandle<()>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ServerHealth {
    pub status: String,
}

#[api_operation(tag = "health", operation_id = "health")]
async fn health() -> Result<web::Json<ServerHealth>, ApiError> {
    Ok(web::Json(ServerHealth {
        status: "ok".to_string(),
    }))
}

pub fn init_server(
    container: Arc<Container>,
    cfg: Arc<Config>,
    host: String,
    port: u16,
) -> Result<ActixServer, Box<dyn std::error::Error>> {
    let cookie_cfg = crate::config::CookieConfig::from_env().expect("Failed to load cookie configuration");

    let server = HttpServer::new(move || {
        let spec = Spec {
            info: Info {
                title: "Matcha API".to_string(),
                version: "0.1.0".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        App::new()
            .document(spec)
            .wrap(TracingLogger::default())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    Key::derive_from(cookie_cfg.session_secret.as_bytes()),
                )
                .cookie_secure(cookie_cfg.secure)
                .cookie_http_only(cookie_cfg.http_only)
                .cookie_same_site(match cookie_cfg.same_site.as_str() {
                    "strict" => SameSite::Strict,
                    "lax" => SameSite::Lax,
                    "none" => SameSite::None,
                    _ => SameSite::Strict,
                })
                .cookie_name("session".to_string())
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::new(cookie_cfg.session_ttl, 0)))
                .build(),
            )
            .app_data(web::Data::new(cfg.clone()))
            .app_data(web::Data::new(container.auth_service.clone()))
            .app_data(web::Data::new(container.user_service.clone()))
            .app_data(web::Data::new(container.user_profile_service.clone()))
            .app_data(web::Data::new(container.profile_tag_service.clone()))
            .app_data(web::Data::new(container.cdn_service.clone()))
            .app_data(
                web::FormConfig::default().error_handler(|err, _req| ApiError::BadRequest(err.to_string()).into()),
            )
            .app_data(
                web::PathConfig::default().error_handler(|err, _req| ApiError::BadRequest(err.to_string()).into()),
            )
            .app_data(
                web::QueryConfig::default().error_handler(|err, _req| ApiError::BadRequest(err.to_string()).into()),
            )
            .app_data(
                web::JsonConfig::default().error_handler(|err, _req| ApiError::BadRequest(err.to_string()).into()),
            )
            .configure(|app| {
                app.service(
                    scope("/v1")
                        .service(resource("").route(get().to(health)))
                        .configure(|cfg| {
                            crate::presentation::routes::auth_route::config(cfg);
                            crate::presentation::routes::user_route::config(cfg);
                            crate::presentation::routes::cdn_route::config(cfg);
                            crate::presentation::routes::tag_route::config(cfg);
                        }),
                );
            })
            .build_with(
                "/openapi.json",
                BuildConfig::default().with(ScalarConfig::new(&"/docs")),
            )
            .wrap(default_cors(cfg.origins.clone()))
    })
    .bind(format!("{}:{}", host, port))?
    .run();

    let server_handle = tokio::spawn(async move {
        tracing::info!("Starting server on {}:{}", host, port);
        if let Err(e) = server.await {
            tracing::error!("Server error: {}", e);
        }
    });

    Ok(ActixServer { handle: server_handle })
}
