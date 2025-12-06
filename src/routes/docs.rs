use actix_web::{HttpResponse, web};
use serde_json::json;

pub fn configure_docs_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/docs")
            .route("", web::get().to(get_api_docs))
            .route("/endpoints", web::get().to(list_endpoints)),
    );
}

async fn get_api_docs() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Rust REST API",
            "version": "1.0.0",
            "description": "API documentation"
        },
        "paths": {
            "/api/auth/login": {
                "post": {
                    "summary": "Login",
                    "tags": ["Authentication"]
                }
            }
        }
    }))
}

async fn list_endpoints() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "endpoints": [
            {
                "path": "/api/auth/login",
                "method": "POST",
                "protected": false,
                "description": "User login"
            },
            {
                "path": "/api/users",
                "method": "GET",
                "protected": true,
                "description": "Get all users"
            },
            {
                "path": "/api/products",
                "method": "GET",
                "protected": true,
                "description": "Search products"
            }
        ]
    }))
}
