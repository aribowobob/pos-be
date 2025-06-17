use actix_web::HttpRequest;
use log::{info, error, debug};
use crate::models::user::User;
use crate::services::db_service::DbConnectionManager;
use crate::errors::ServiceError;

/// Helper function to extract authentication information from request
/// This function extracts the token, verifies it, and returns the user information
pub async fn extract_auth_user(req: &HttpRequest, db_manager: &DbConnectionManager) 
    -> Result<(User, i32), ServiceError> {
    
    // Log all cookies for debugging
    if let Ok(cookies) = req.cookies() {
        debug!("Found {} cookies in request", cookies.len());
        for cookie in cookies.iter() {
            debug!("Cookie: {} = {}", cookie.name(), cookie.value());
        }
    } else {
        debug!("No cookies found in request");
    }
    
    // Extract token from cookie or authorization header
    let token = req.cookie("access_token")
        .map(|c| {
            debug!("Found access_token cookie: {}", c.value());
            c.value().to_string()
        })
        .or_else(|| {
            debug!("No access_token cookie found, checking Authorization header");
            req.headers()
                .get(actix_web::http::header::AUTHORIZATION)
                .and_then(|auth| auth.to_str().ok())
                .and_then(|auth_str| {
                    if auth_str.starts_with("Bearer ") {
                        debug!("Found Bearer token in Authorization header");
                        Some(auth_str[7..].to_string())
                    } else {
                        debug!("Authorization header doesn't contain Bearer token");
                        None
                    }
                })
        });
    
    // Check if token exists
    let token = match token {
        Some(token) => token,
        None => {
            error!("No authentication token provided");
            return Err(ServiceError::Unauthorized);
        }
    };
    
    // Verify token and extract user info
    let token_data = match crate::services::auth::verify_jwt(&token) {
        Ok(token_data) => {
            debug!("Token verified successfully, email: {}", token_data.claims.email);
            token_data
        },
        Err(e) => {
            error!("Invalid authentication token: {:?}", e);
            return Err(ServiceError::Unauthorized);
        }
    };
    
    // Get user from database using email from token
    let email = token_data.claims.email;
    
    // Get database pool
    let pool = match db_manager.get_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to get database connection: {:?}", e);
            return Err(ServiceError::DatabaseConnectionError);
        }
    };
    
    // Get user from database
    let user = match crate::services::auth::get_user_by_email(&pool, &email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            error!("User not found for email: {}", email);
            return Err(ServiceError::Unauthorized);
        }
        Err(e) => {
            error!("Database error when fetching user: {:?}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };
    
    // Extract company_id from user
    let company_id = user.company_id;
    debug!("Authenticated user: {} (company_id: {})", user.email, company_id);
    
    Ok((user, company_id))
}
