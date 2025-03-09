use crate::models::{response::ApiResponse, AppState};
use crate::services::auth::verify_jwt;
use crate::services::user_service::get_user_with_stores;
use actix_web::{web, HttpRequest, HttpResponse};
use log::{error, info};

pub async fn get_user(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    // Extract token from cookie
    let cookie = match req.cookie("access_token") {
        Some(c) => c,
        None => {
            error!("No access_token cookie found in request");
            return HttpResponse::Unauthorized().json(ApiResponse::<serde_json::Value>::error(
                "No authentication token provided",
            ));
        }
    };

    let token = cookie.value();
    info!("Processing get_user request for token");

    // Verify and decode the JWT token
    let token_data = match verify_jwt(token) {
        Ok(data) => data,
        Err(e) => {
            error!("JWT verification failed: {:?}", e);
            return HttpResponse::Unauthorized().json(ApiResponse::<serde_json::Value>::error(
                &format!("Invalid token: {:?}", e),
            ));
        }
    };

    let email = token_data.claims.email;
    info!("Fetching user data for email: {}", email);

    // Get user with stores from database using our service
    match get_user_with_stores(&data.db, email).await {
        Ok(user_with_stores) => {
            info!("Successfully retrieved user data");
            HttpResponse::Ok().json(ApiResponse::success(user_with_stores))
        }
        Err(e) => {
            error!("Failed to get user data: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<serde_json::Value>::error(
                &format!("Failed to retrieve user data: {:?}", e),
            ))
        }
    }
}
