use matcha_back::config::Config;
use matcha_back::server::container::Container;
use matcha_back::server::init::init_server;
use matcha_back::tracing::init::{init_service_logging, init_telemetry};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = Config::from_env().expect("Failed to load configuration");

    match cfg.telemetry_collector_endpoint {
        Some(ref endpoint) => init_telemetry(endpoint),
        None => init_service_logging(),
    }

    tracing::debug!("Configuration: {:?}", cfg);

    let container = Container::new(&cfg.database_url, &cfg.redis_url, &cfg.base_url).await;

    let port = cfg.port;
    let host = cfg.host.clone();

    let server = init_server(Arc::new(container), Arc::new(cfg), host, port).expect("Failed to init server");

    server.handle.await?;

    Ok(())
}
