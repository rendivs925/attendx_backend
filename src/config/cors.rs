use actix_cors::Cors;

pub fn configure_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_header()
        .allow_any_method()
        .supports_credentials()
        .max_age(3600)
}
