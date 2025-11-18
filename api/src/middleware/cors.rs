use actix_cors::Cors;
use actix_web::http;
use std::env;

pub fn configure_cors() -> Cors {
    let allowed_origin = env::var("CORS_ALLOWED_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    Cors::default()
        .allowed_origin(&allowed_origin)
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ])
        .max_age(3600)
}
