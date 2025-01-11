use std::sync::LazyLock;

use opentelemetry::trace::TracerProvider;
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::runtime::TokioCurrentThread;
use opentelemetry_sdk::trace::Config;
use opentelemetry_sdk::Resource;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::fmt::Subscriber;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

const APP_NAME: &str = "matcha-back";

static RESOURCE: LazyLock<Resource> = LazyLock::new(|| Resource::new(vec![KeyValue::new("service.name", APP_NAME)]));

pub fn init_telemetry(collector_endpoint: &str) {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(collector_endpoint),
        )
        .with_trace_config(Config::default().with_resource(RESOURCE.clone()))
        .install_batch(TokioCurrentThread)
        .expect("Failed to install OpenTelemetry tracer.")
        .tracer_builder(APP_NAME)
        .build();

    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let formatting_layer = BunyanFormattingLayer::new(APP_NAME.into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    let _ = tracing::subscriber::set_global_default(subscriber).is_err();
    {
        eprintln!("Global tracing subscriber is already set; skipping telemetry initialization.");
    }
}

pub fn init_service_logging() {
    let log_level = match std::env::var("RUST_LOG").unwrap_or("info".to_string()).as_str() {
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };

    let subscriber = Subscriber::builder().with_max_level(log_level).finish();

    let _ = tracing::subscriber::set_global_default(subscriber).is_err();
    {
        eprintln!("Global tracing subscriber is already set; skipping telemetry initialization.");
    }
}
