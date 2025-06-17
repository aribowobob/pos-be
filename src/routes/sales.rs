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
                web::resource("/cart/clear")
                    .route(web::delete().to(sales::clear_cart))  // Add DELETE route for clearing cart
            )
            .service(
                web::resource("/cart/{id}")
                    .route(web::delete().to(sales::delete_from_cart))
                    .route(web::put().to(sales::update_cart_item))
            )
            .service(
                web::resource("/orders")
                    .route(web::post().to(sales::create_order))  // Add POST route for creating orders
            )
            .service(
                web::resource("/orders/{id}")
                    .route(web::get().to(sales::get_sales_order_by_id))  // Add GET route for fetching order by ID
            )
            .service(
                web::resource("/report")
                    .route(web::get().to(sales::get_sales_report))  // Add GET route for sales report
            )
    );
}
