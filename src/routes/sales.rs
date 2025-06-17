use actix_web::web;
use crate::handlers::sales;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sales")
            .service(
                web::resource("/cart")
                    .route(web::post().to(sales::add_to_cart))
            )
            .service(
                web::resource("/cart/{id}")
                    .route(web::delete().to(sales::delete_from_cart))
            )
    );
}
