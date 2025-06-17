use actix_web::web;
use crate::handlers::product::{get_product_categories, create_product, get_products};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("/categories", web::get().to(get_product_categories))
            .route("", web::post().to(create_product))
            .route("", web::get().to(get_products))
            // The following routes are for future implementation
            // .route("/{id}", web::get().to(get_product_by_id))
    );
}
