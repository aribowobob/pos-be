use actix_web::{web, HttpResponse, Responder};

async fn get_all_products() -> impl Responder {
    // Placeholder for product retrieval logic
    HttpResponse::Ok().json(serde_json::json!({
        "products": []
    }))
}

async fn get_product_by_id(path: web::Path<(String,)>) -> impl Responder {
    let product_id = &path.0;
    // Placeholder for single product retrieval logic
    HttpResponse::Ok().json(serde_json::json!({
        "id": product_id,
        "name": "Example Product"
    }))
}

// Configure product routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::get().to(get_all_products))
            .route("/{id}", web::get().to(get_product_by_id)),
    );
}
