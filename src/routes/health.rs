use actix_web::{web, HttpResponse, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Service is running"
    }))
}

// Configure health check routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health").route("", web::get().to(health_check)));
}
