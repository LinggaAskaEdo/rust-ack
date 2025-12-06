use crate::controllers;
use actix_web::web;

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(controllers::user::create_user))
            .route("", web::get().to(controllers::user::get_all_users))
            .route("/{id}", web::get().to(controllers::user::get_user))
            .route("/{id}", web::put().to(controllers::user::update_user))
            .route("/{id}", web::delete().to(controllers::user::delete_user)),
    );
}
