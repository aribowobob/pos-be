use crate::models::{response::ApiResponse, AppState};
use crate::services::auth::{create_jwt, get_user_by_email, verify_google_token};
use crate::services::db_service::DbConnectionManager;
use actix_web::{cookie::Cookie, web, HttpResponse};
use log::{error, info, debug};
use serde::Deserialize;
use std::env;
use time::Duration; // Use time::Duration instead of std::time::Duration

// Helper function to create auth cookie - add #[allow(dead_code)] to suppress the warning
#[allow(dead_code)]
fn create_auth_cookie(token: &str) -> Cookie {
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

    let mut cookie = Cookie::build("access_token", token.to_owned())
        .path("/")
        .http_only(true)
        .max_age(actix_web::cookie::time::Duration::hours(4));

    if environment == "production" {
        cookie = cookie
            .secure(true)
            .same_site(actix_web::cookie::SameSite::Strict);
    } else {
        cookie = cookie
            .secure(true)
            .same_site(actix_web::cookie::SameSite::None);
    }

    cookie.finish()
}

#[derive(Deserialize)]
pub struct TokenRequest {
    #[serde(rename = "token")]
    access_token: String, // Renamed to make it used
}

pub async fn google_login(
    token_req: web::Json<TokenRequest>,
    data: web::Data<AppState>,
) -> HttpResponse {
    info!("Processing Google login");

    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
    
    // Verify Google token
    let user_info_result = verify_google_token(&token_req.access_token).await;

    match user_info_result {
        Ok(user_info) => {
            info!("Google token verified for: {}", user_info.email);
            
            // Create database manager
            let db_manager = DbConnectionManager::new(data.db_connection_string.clone());
            debug!("Database connection string: {}", data.db_connection_string);
            
            let pool = match db_manager.get_pool().await {
                Ok(pool) => pool,
                Err(e) => {
                    error!("Failed to get database pool: {:?}", e);
                    return HttpResponse::ServiceUnavailable().json(ApiResponse::<serde_json::Value>::error(
                        "Database connection failed"
                    ));
                }
            };

            // Check if user exists in our database
            match get_user_by_email(&pool, &user_info.email).await {
                Ok(Some(user)) => {
                    info!("User found in database: {}", user.email);
                    
                    // Create JWT token
                    let token = create_jwt(&user_info);
                    
                    // Set secure flag based on environment
                    let is_secure = environment != "development";
                    
                    // Create cookie with the JWT token
                    let cookie = Cookie::build("access_token", token.clone())
                        .path("/")
                        .max_age(Duration::hours(24)) // Use time::Duration::hours
                        .http_only(true)
                        .secure(is_secure)
                        .finish();
                    
                    // Return successful response with cookie
                    HttpResponse::Ok()
                        .cookie(cookie)
                        .json(ApiResponse {
                            status: "success".to_string(),
                            message: "Login successful".to_string(),
                            data: Some(serde_json::json!({
                                "token": token
                            })),
                        })
                }
                Ok(None) => {
                    info!("User not found in database: {}", user_info.email);
                    HttpResponse::Unauthorized().json(ApiResponse::<serde_json::Value>::error(
                        "User not registered in our system"
                    ))
                }
                Err(e) => {
                    error!("Database error while checking user: {}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<serde_json::Value>::error(
                        &format!("Database error: {}", e)
                    ))
                }
            }
        }
        Err(e) => {
            error!("Failed to verify Google token: {}", e);
            HttpResponse::Unauthorized().json(ApiResponse::<serde_json::Value>::error(
                &format!("Invalid Google token: {}", e)
            ))
        }
    }
}

pub async fn logout() -> HttpResponse {
    // Create a cookie with the same name but empty value and immediate expiration
    let cookie = Cookie::build("access_token", "")
        .path("/")
        .max_age(Duration::seconds(0)) // Use time::Duration::seconds instead
        .http_only(true)
        .secure(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(ApiResponse {
            status: "success".to_string(),
            message: "Logged out successfully".to_string(),
            data: None::<serde_json::Value>,
        })
}
