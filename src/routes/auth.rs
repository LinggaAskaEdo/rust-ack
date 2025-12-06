use crate::controllers;
use actix_web::web;

pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(controllers::auth::login))
            .route("/logout", web::post().to(controllers::auth::logout)),
    );
}
