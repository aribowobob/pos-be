use crate::models::{response::ApiResponse, AppState, auth::TokenRequest};
use crate::services::auth::{create_jwt, get_user_by_email};
use crate::services::db_service::DbConnectionManager;
use actix_web::{cookie::Cookie, web, HttpResponse};
use log::{error, info, debug};
use serde::Deserialize;
use std::env;
use time::Duration; // Use time::Duration instead of std::time::Duration

// Helper function to extract domain from the last FRONTEND_URL for production
fn get_cookie_domain() -> Option<String> {
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
    
    if environment != "production" {
        return None; // No domain restriction for development
    }
    
    // Get the last FRONTEND_URL for production domain
    let frontend_urls = env::var("FRONTEND_URLS").unwrap_or_default();
    let urls: Vec<&str> = frontend_urls.split(',').collect();
    
    if let Some(last_url) = urls.last() {
        let url = last_url.trim();
        
        // Extract host from URL (remove protocol and path)
        let host = if url.starts_with("https://") {
            &url[8..]
        } else if url.starts_with("http://") {
            &url[7..]
        } else {
            url
        };
        
        // Remove path if any
        let host = host.split('/').next().unwrap_or(host);
        
        // Extract the main domain (e.g., from "pos.opense7en.com" get ".opense7en.com")
        let parts: Vec<&str> = host.split('.').collect();
        if parts.len() >= 2 {
            let main_domain = format!(".{}", parts[parts.len()-2..].join("."));
            info!("Setting cookie domain for production: {}", main_domain);
            return Some(main_domain);
        }
    }
    
    None
}

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

/// Authenticate with Google
///
/// Validates Google OAuth token and returns JWT token
#[utoipa::path(
    post,
    path = "/auth/google",
    request_body(content = TokenRequest, description = "Google OAuth token", content_type = "application/json"),
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<String>),
        (status = 401, description = "Authentication failed", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    tag = "auth"
)]
pub async fn google_login(
    token_req: web::Json<TokenRequest>,
    data: web::Data<AppState>,
) -> HttpResponse {
    info!("Processing Google login");
    debug!("Received token request: {:?}", token_req);
    debug!("Token value: '{}'", token_req.access_token);
    
    // Verify Google token using the improved verification function
    let user_info_result = crate::services::google_auth::verify_google_token(&token_req.access_token).await;

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
                    
                    // Untuk permintaan cross-origin dengan SameSite=None, cookie harus selalu secure
                    // Ini adalah persyaratan browser modern, meskipun API berjalan di HTTP
                    
                    info!("Setting cookie for cross-origin support");
                    
                    // Build cookie with optional domain based on environment
                    let mut cookie_builder = Cookie::build("access_token", token.clone())
                        .path("/")
                        .max_age(Duration::hours(24))
                        .http_only(true)
                        .same_site(actix_web::cookie::SameSite::None) // Required for cross-site requests
                        .secure(true); // Must be true when SameSite=None, even if using HTTP
                    
                    // Set domain for production environment
                    if let Some(domain) = get_cookie_domain() {
                        cookie_builder = cookie_builder.domain(domain.clone());
                        debug!("Cookie domain set to: {}", domain);
                    } else {
                        debug!("No domain restriction for cookie (development mode)");
                    }
                    
                    let cookie = cookie_builder.finish();
                    
                    debug!("Cookie path: {}, SameSite: None, Secure: true", cookie.path().unwrap_or("/"));
                    
                    // Return successful response with cookie and additional headers for CORS
                    HttpResponse::Ok()
                        .append_header(("Access-Control-Allow-Credentials", "true"))
                        .append_header(("Access-Control-Expose-Headers", "Set-Cookie"))
                        .cookie(cookie)
                        .json(ApiResponse {
                            status: "success".to_string(),
                            message: "Login successful".to_string(),
                            data: Some(serde_json::json!({
                                "token": token,
                                "cookies_enabled": true // Flag to indicate cookies are being used
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

/// Logout user
///
/// Clears authentication cookie
#[utoipa::path(
    post,
    path = "/auth/logout",
    responses(
        (status = 200, description = "Logout successful", body = ApiResponse<String>)
    ),
    tag = "auth"
)]
pub async fn logout() -> HttpResponse {
    // Build cookie with appropriate settings, matching login
    let mut cookie_builder = Cookie::build("access_token", "")
        .path("/")
        .max_age(Duration::seconds(0))
        .http_only(true)
        .same_site(actix_web::cookie::SameSite::None) // Consistent with login cookie
        .secure(true); // Must be true when SameSite=None, even if using HTTP
    
    // Set domain for production environment (same as login)
    if let Some(domain) = get_cookie_domain() {
        cookie_builder = cookie_builder.domain(domain);
    }
    
    let cookie = cookie_builder.finish();

    HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Credentials", "true"))
        .append_header(("Access-Control-Expose-Headers", "Set-Cookie"))
        .cookie(cookie)
        .json(ApiResponse {
            status: "success".to_string(),
            message: "Logged out successfully".to_string(),
            data: None::<serde_json::Value>,
        })
}
