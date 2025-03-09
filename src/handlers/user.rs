use crate::models::response::ApiResponse;
use actix_web::HttpResponse;

pub async fn get_user() -> HttpResponse {
    // For now, just return a simple response with data: true
    HttpResponse::Ok().json(ApiResponse::success(true))
}
