use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod configs;
mod controllers;
mod dao;
mod middleware;
mod models;
mod routes;
mod services;

use configs::{AppState, logging::LoggingConfig};
use middleware::{CorsMiddleware, LoggingMiddleware, RateLimitMiddleware};
use routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Initialize logging
    let logging_config = LoggingConfig::from_env();
    logging_config.init();

    tracing::info!("🚀 Starting Rust REST API");

    // Initialize application state
    let state = match AppState::new().await {
        Ok(state) => {
            tracing::info!("✓ Application state initialized");
            state
        }
        Err(e) => {
            tracing::error!("Failed to initialize application state: {}", e);
            std::process::exit(1);
        }
    };

    let bind_address = "0.0.0.0:8080";
    tracing::info!("🚀 Starting server at http://{}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(state.clone()))
            .wrap(CorsMiddleware::from_env()) // ← CORS middleware
            .wrap(Logger::default()) // ← Actix's built-in logger
            .wrap(LoggingMiddleware) // ← Custom logging middleware
            .wrap(RateLimitMiddleware::new(100)) // 100 requests per minute
            .configure(configure_routes)
    })
    .bind(bind_address)?
    .run()
    .await
}
