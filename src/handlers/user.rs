use crate::models::{response::ApiResponse, AppState};
use crate::services::auth::verify_jwt;
use crate::services::db_service::DbConnectionManager;
use crate::services::user_service;
use actix_web::{web, HttpRequest, HttpResponse};
use log::{error, info};

use crate::errors::ServiceError;

/// Get current user information
///
/// Returns the current authenticated user's profile and associated stores
#[utoipa::path(
    get,
    path = "/api/users/get-user",
    responses(
        (status = 200, description = "User profile retrieved successfully", body = ApiResponse<String>),
        (status = 401, description = "Authentication required", body = ApiResponse<()>),
        (status = 404, description = "User not found", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    security(
        ("cookie_auth" = [])
    ),
    tag = "users"
)]
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

    // Create database connection manager on-demand
    let db_manager = DbConnectionManager::new(data.db_connection_string.clone());
    
    // Get user with stores from database using our service
    match user_service::get_user_with_stores(&db_manager, email.clone()).await {
        Ok(user_with_stores) => {
            info!("Successfully retrieved user data");
            HttpResponse::Ok().json(ApiResponse::success(user_with_stores))
        }
        Err(ServiceError::DatabaseConnectionError) => {
            error!("Database unavailable, returning error response for user {}", email);
            HttpResponse::ServiceUnavailable().json(ApiResponse::<serde_json::Value>::error(
                "Database service is currently unavailable",
            ))
        }
        Err(ServiceError::NotFound) => {
            HttpResponse::NotFound().json(ApiResponse::<serde_json::Value>::error(
                &format!("User not found: {}", email),
            ))
        }
        Err(e) => {
            error!("Failed to get user data: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<serde_json::Value>::error(
                &format!("Failed to retrieve user data: {:?}", e),
            ))
        }
    }
}
