use actix_cors::Cors;

pub fn default_cors(origins: Vec<String>) -> Cors {
    Cors::default()
        .allowed_origin_fn(move |origin, _req_head| origins.iter().any(|o| o == origin))
        .allow_any_header()
        .allow_any_method()
        .supports_credentials()
        .max_age(3600)
        .send_wildcard()
}
