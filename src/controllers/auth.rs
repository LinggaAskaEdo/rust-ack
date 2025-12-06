use actix_web::{Error, HttpRequest, HttpResponse, web};

use crate::configs::AppState;
use crate::models::LoginDto;
use crate::services::AuthService;

pub async fn login(
    state: web::Data<AppState>,
    dto: web::Json<LoginDto>,
) -> Result<HttpResponse, Error> {
    tracing::info!("Login attempt for user: {}", dto.username);

    match AuthService::login(&state, dto.into_inner()).await {
        Ok(token) => {
            tracing::info!(
                "Login successful for user: {}",
                token.token.chars().take(10).collect::<String>()
            );
            Ok(HttpResponse::Ok().json(token))
        }
        Err(e) => {
            tracing::warn!("Login failed: {}", e);
            Ok(HttpResponse::Unauthorized().json(serde_json::json!({"error": e})))
        }
    }
}

pub async fn logout(state: web::Data<AppState>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .unwrap_or("");

    tracing::info!("Logout attempt");

    match AuthService::logout(&state, token).await {
        Ok(_) => {
            tracing::info!("Logout successful");
            Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Logged out successfully"})))
        }
        Err(e) => {
            tracing::error!("Logout failed: {}", e);
            Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": e})))
        }
    }
}
