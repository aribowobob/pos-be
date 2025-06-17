use actix_web::web;
use crate::handlers::product::{get_product_categories, create_product};
use crate::middleware::skip_auth::SkipAuth;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("/categories", web::get().to(get_product_categories).wrap(SkipAuth))
            .route("", web::post().to(create_product)) // POST endpoint for creating products (requires auth)
            // The following routes are for future implementation
            // .route("", web::get().to(get_all_products))
            // .route("/{id}", web::get().to(get_product_by_id))
    );
}

// Future implementation placeholders:
/*
async fn get_all_products() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "products": []
    }))
}

async fn get_product_by_id(path: web::Path<(String,)>) -> impl Responder {
    let product_id = &path.0;
    HttpResponse::Ok().json(serde_json::json!({
        "id": product_id,
        "name": "Example Product"
    }))
}
*/
