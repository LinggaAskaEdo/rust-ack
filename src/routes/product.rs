use crate::controllers;
use actix_web::web;

pub fn configure_product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::post().to(controllers::product::create_product))
            .route("", web::get().to(controllers::product::search_products))
            .route("/{id}", web::get().to(controllers::product::get_product))
            .route("/{id}", web::put().to(controllers::product::update_product))
            .route(
                "/{id}",
                web::delete().to(controllers::product::delete_product),
            ),
    );
}
