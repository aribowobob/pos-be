// file: /Users/catalyst/Documents/playground/pos-be/src/handlers/product.rs
use crate::models::{AppState, response::ApiResponse};
use crate::models::product::{ProductCategoryQueryParams, NewProduct};
use crate::services::db_service::DbConnectionManager;
use crate::services::product_service;
use actix_web::{web, HttpResponse};
use log::{error, info};

pub async fn get_product_categories(
    query: web::Query<ProductCategoryQueryParams>,
    data: web::Data<AppState>,
) -> HttpResponse {
    info!("Processing get_product_categories request");
    
    // Create database connection manager
    let db_manager = DbConnectionManager::new(data.db_connection_string.clone());
    
    // Call the service to get product categories
    match product_service::get_product_categories(
        &db_manager,
        query.search.clone(),
        query.page,
        query.size,
    ).await {
        Ok(categories) => {
            info!("Successfully retrieved product categories");
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "message": "Product categories retrieved successfully",
                "data": categories
            }))
        }
        Err(e) => {
            error!("Failed to retrieve product categories: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to retrieve product categories: {:?}", e)
            }))
        }
    }
}

pub async fn create_product(
    req: web::HttpRequest,
    data: web::Data<AppState>,
    product_data: web::Json<NewProduct>,
) -> HttpResponse {
    info!("Processing create_product request");
    
    // Extract user info from token
    let token = req.cookie("access_token")
        .map(|c| c.value().to_string())
        .or_else(|| {
            req.headers()
                .get(actix_web::http::header::AUTHORIZATION)
                .and_then(|auth| auth.to_str().ok())
                .and_then(|auth_str| {
                    if auth_str.starts_with("Bearer ") {
                        Some(auth_str[7..].to_string())
                    } else {
                        None
                    }
                })
        });
    
    let token = match token {
        Some(token) => token,
        None => {
            error!("No authentication token provided");
            return HttpResponse::Unauthorized().json(ApiResponse {
                status: "error".to_string(),
                message: "No authentication token provided".to_string(),
                data: None::<()>,
            });
        }
    };
    
    // Verify token and extract user info
    let token_data = match crate::services::auth::verify_jwt(&token) {
        Ok(token_data) => token_data,
        Err(e) => {
            error!("Invalid authentication token: {:?}", e);
            return HttpResponse::Unauthorized().json(ApiResponse {
                status: "error".to_string(),
                message: "Invalid authentication token".to_string(),
                data: None::<()>,
            });
        }
    };
    
    // Create database connection manager
    let db_manager = DbConnectionManager::new(data.db_connection_string.clone());
    
    // Get user info from database
    let pool = match db_manager.get_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to get database connection: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: "Database connection error".to_string(),
                data: None::<()>,
            });
        }
    };
    
    // Get user from database using email from token
    let email = token_data.claims.email;
    let user = match crate::services::auth::get_user_by_email(&pool, &email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            error!("User not found for email: {}", email);
            return HttpResponse::Unauthorized().json(ApiResponse {
                status: "error".to_string(),
                message: "User not found".to_string(),
                data: None::<()>,
            });
        }
        Err(e) => {
            error!("Database error when fetching user: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: "Database error".to_string(),
                data: None::<()>,
            });
        }
    };
    
    // Extract company_id from user
    let company_id = user.company_id;
    
    // Create product with company_id from user
    let mut product = product_data.into_inner();
    
    // Call the service to create the product
    match product_service::create_product(
        &db_manager,
        product,
        company_id, // Pass the company_id from the authenticated user
    ).await {
        Ok(product) => {
            info!("Product created successfully: ID {}", product.id);
            HttpResponse::Created().json(ApiResponse {
                status: "success".to_string(),
                message: "Product created successfully".to_string(),
                data: Some(product),
            })
        }
        Err(e) => {
            error!("Failed to create product: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: format!("Failed to create product: {:?}", e),
                data: None::<()>,
            })
        }
    }
}
