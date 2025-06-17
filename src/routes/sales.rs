use actix_web::web;
use crate::handlers::sales;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sales")
            .service(
                web::resource("/cart")
                    .route(web::post().to(sales::add_to_cart))
                    .route(web::get().to(sales::get_cart_items))  // Add GET route for fetching cart items
            )
            .service(
                web::resource("/cart/{id}")
                    .route(web::delete().to(sales::delete_from_cart))
                    .route(web::put().to(sales::update_cart_item))
            )
    );
}
