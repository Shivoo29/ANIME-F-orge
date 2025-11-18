pub mod animations;
pub mod auth;
pub mod marketplace;
pub mod search;
pub mod users;

use actix_web::{web, HttpResponse, Responder};

/// Health check endpoint
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "AnimaForge API",
        "version": "0.1.0"
    }))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/health", web::get().to(health_check))
            .configure(auth::configure)
            .configure(animations::configure)
            .configure(users::configure)
            .configure(marketplace::configure)
            .configure(search::configure),
    );
}
