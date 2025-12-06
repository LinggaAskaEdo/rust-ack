use crate::configs::AppState;
use crate::middleware::AuthMiddleware;
use actix_web::HttpResponse;
use actix_web::web;

mod auth;
mod docs;
mod product;
mod user;

pub use auth::configure_auth_routes;
pub use docs::configure_docs_routes;
pub use product::configure_product_routes;
pub use user::configure_user_routes;

// Health check endpoint
async fn health_check(state: web::Data<AppState>) -> HttpResponse {
    match state.health_check().await {
        Ok(status) => {
            let http_status = if status.status == "healthy" {
                actix_web::http::StatusCode::OK
            } else {
                actix_web::http::StatusCode::SERVICE_UNAVAILABLE
            };
            HttpResponse::build(http_status).json(status)
        }
        Err(e) => HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "error",
            "message": format!("Health check failed: {}", e)
        })),
    }
}

// Main router configuration
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health_check))
            .configure(configure_auth_routes)
            .configure(configure_docs_routes)
            .service(
                web::scope("")
                    .wrap(AuthMiddleware)
                    .configure(configure_user_routes)
                    .configure(configure_product_routes),
            ),
    );
}
