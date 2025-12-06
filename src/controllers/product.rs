use actix_web::{Error, HttpRequest, HttpResponse, web};
use uuid::Uuid;

use crate::configs::AppState;
use crate::models::{CreateProductDto, ProductQuery, UpdateProductDto};
use crate::services::{AuthService, ProductService};

pub async fn create_product(
    state: web::Data<AppState>,
    req: HttpRequest,
    dto: web::Json<CreateProductDto>,
) -> Result<HttpResponse, Error> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .unwrap_or("");

    let claims = AuthService::validate_token(&state, token).await.unwrap();
    let user_id = Uuid::parse_str(&claims.user_id).unwrap();

    match ProductService::create(&state.db, dto.into_inner(), user_id).await {
        Ok(product) => Ok(HttpResponse::Created().json(product)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": e}))),
    }
}

pub async fn get_product(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    match ProductService::get_by_id(&state.db, id.into_inner()).await {
        Ok(product) => Ok(HttpResponse::Ok().json(product)),
        Err(e) => Ok(HttpResponse::NotFound().json(serde_json::json!({"error": e}))),
    }
}

pub async fn search_products(
    state: web::Data<AppState>,
    query: web::Query<ProductQuery>,
) -> Result<HttpResponse, Error> {
    match ProductService::search(&state.db, query.into_inner()).await {
        Ok(products) => Ok(HttpResponse::Ok().json(products)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({"error": e}))),
    }
}

pub async fn update_product(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    dto: web::Json<UpdateProductDto>,
) -> Result<HttpResponse, Error> {
    match ProductService::update(&state.db, id.into_inner(), dto.into_inner()).await {
        Ok(product) => Ok(HttpResponse::Ok().json(product)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": e}))),
    }
}

pub async fn delete_product(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    match ProductService::delete(&state.db, id.into_inner()).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": e}))),
    }
}
