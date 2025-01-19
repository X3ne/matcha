pub mod init;

#[macro_export]
macro_rules! trace_peer_infos {
    ($peer_infos:expr) => {
        let span = tracing::span!(
            tracing::Level::INFO,
            "product_infos",
            product = ?$peer_infos.product,
            version = ?$peer_infos.version,
            platform = ?$peer_infos.platform
        );
        let _enter = span.enter();
    };
}
