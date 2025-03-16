use actix_web::web;

// Remove the unused functions and leave only the configure function
pub fn configure(_cfg: &mut web::ServiceConfig) {
    // Products routes will be implemented in the future
    // Example implementation (commented out until needed):
    /*
    _cfg.service(
        web::scope("/products")
            .route("", web::get().to(get_all_products))
            .route("/{id}", web::get().to(get_product_by_id))
    );
    */
}

// When needed, uncomment and implement these functions:
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
