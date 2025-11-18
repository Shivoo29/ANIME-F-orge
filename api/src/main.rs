mod db;
mod middleware;
mod models;
mod routes;
mod utils;

use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use log::info;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize logger
    env_logger::init();

    // Get configuration from environment
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Create database connection pool
    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");

    info!("Starting AnimaForge API server at {}:{}", host, port);
    info!("Health check available at http://{}:{}/api/v1/health", host, port);

    // Start HTTP server
    HttpServer::new(move || {
        // Configure JWT authentication middleware
        let auth_middleware = HttpAuthentication::bearer(middleware::jwt_validator);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(middleware::configure_cors())
            // Public routes (no authentication required)
            .service(
                web::scope("/api/v1")
                    .route("/health", web::get().to(routes::health_check))
                    .configure(routes::auth::configure)
                    .configure(routes::marketplace::configure)
                    .configure(routes::search::configure)
                    .service(
                        web::scope("/animations")
                            .route("", web::get().to(routes::animations::list_animations))
                            .route("/{id}", web::get().to(routes::animations::get_animation))
                            .route("/{id}/download", web::get().to(routes::animations::download_animation)),
                    )
                    .service(
                        web::scope("/users")
                            .route("/{id}/profile", web::get().to(routes::users::get_profile))
                            .route("/{id}/animations", web::get().to(routes::users::get_user_animations)),
                    ),
            )
            // Protected routes (authentication required)
            .service(
                web::scope("/api/v1")
                    .wrap(auth_middleware)
                    .service(
                        web::scope("/animations")
                            .route("", web::post().to(routes::animations::create_animation))
                            .route("/{id}", web::put().to(routes::animations::update_animation))
                            .route("/{id}", web::delete().to(routes::animations::delete_animation)),
                    )
                    .service(
                        web::scope("/users")
                            .route("/{id}/profile", web::put().to(routes::users::update_profile)),
                    ),
            )
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
