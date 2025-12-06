use actix_web::{Error, HttpResponse, web};
use uuid::Uuid;

use crate::configs::AppState;
use crate::models::{CreateUserDto, UpdateUserDto};
use crate::services::UserService;

pub async fn create_user(
    state: web::Data<AppState>,
    dto: web::Json<CreateUserDto>,
) -> Result<HttpResponse, Error> {
    match UserService::create(&state.db, dto.into_inner()).await {
        Ok(user) => Ok(HttpResponse::Created().json(user)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": e}))),
    }
}

pub async fn get_user(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    match UserService::get_by_id(&state.db, id.into_inner()).await {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Ok(HttpResponse::NotFound().json(serde_json::json!({"error": e}))),
    }
}

pub async fn get_all_users(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    match UserService::get_all(&state.db).await {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({"error": e}))),
    }
}

pub async fn update_user(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    dto: web::Json<UpdateUserDto>,
) -> Result<HttpResponse, Error> {
    match UserService::update(&state.db, id.into_inner(), dto.into_inner()).await {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": e}))),
    }
}

pub async fn delete_user(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    match UserService::delete(&state.db, id.into_inner()).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": e}))),
    }
}
