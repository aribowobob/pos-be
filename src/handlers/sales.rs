use crate::models::{AppState, response::ApiResponse};
use crate::models::sales::NewSalesCart;
use crate::services::db_service::DbConnectionManager;
use crate::services::sales_service;
use actix_web::{web, HttpResponse, HttpRequest};
use log::{error, info};

pub async fn add_to_cart(
    req: HttpRequest,
    data: web::Data<AppState>,
    cart_data: web::Json<NewSalesCart>,
) -> HttpResponse {
    info!("Processing add_to_cart request");
    
    // Create database connection manager
    let db_manager = DbConnectionManager::new(data.db_connection_string.clone());
    
    // Extract authentication using our helper function
    let auth_result = crate::middleware::extract_auth::extract_auth_user(&req, &db_manager).await;
    
    // Handle authentication result
    let (user, _company_id) = match auth_result {
        Ok((user, company_id)) => {
            info!("User authenticated: {} (company_id: {})", user.email, company_id);
            (user, company_id)
        },
        Err(e) => {
            error!("Authentication failed: {:?}", e);
            return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(&format!("Authentication failed: {}", e)));
        }
    };
    
    // Process the request with the authenticated user's ID
    match sales_service::add_to_cart(&db_manager, cart_data.into_inner(), user.id).await {
        Ok(cart_item) => {
            info!("Item added to cart successfully with ID: {}", cart_item.id);
            HttpResponse::Created().json(ApiResponse::success(cart_item))
        },
        Err(e) => {
            error!("Failed to add item to cart: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&format!("Failed to add item to cart: {}", e)))
        }
    }
}

pub async fn delete_from_cart(
    req: HttpRequest,
    data: web::Data<AppState>,
    path: web::Path<(i32,)>, // Cart item ID from path
) -> HttpResponse {
    info!("Processing delete_from_cart request for item ID: {}", path.0);
    
    // Create database connection manager
    let db_manager = DbConnectionManager::new(data.db_connection_string.clone());
    
    // Extract authentication using helper function
    let auth_result = crate::middleware::extract_auth::extract_auth_user(&req, &db_manager).await;
    
    // Handle authentication result
    let (user, _company_id) = match auth_result {
        Ok((user, company_id)) => {
            info!("User authenticated: {} (company_id: {})", user.email, company_id);
            (user, company_id)
        },
        Err(e) => {
            error!("Authentication failed: {:?}", e);
            return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(&format!("Authentication failed: {}", e)));
        }
    };
    
    // Process the delete request with the authenticated user's ID
    match sales_service::delete_from_cart(&db_manager, path.0, user.id).await {
        Ok(deleted) => {
            if deleted {
                info!("Item deleted from cart successfully with ID: {}", path.0);
                HttpResponse::Ok().json(ApiResponse::success("Item deleted successfully"))
            } else {
                info!("No item found with ID: {} for user ID: {}", path.0, user.id);
                HttpResponse::NotFound().json(ApiResponse::<()>::error("Item not found or not owned by this user"))
            }
        },
        Err(e) => {
            error!("Failed to delete item from cart: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&format!("Failed to delete item from cart: {}", e)))
        }
    }
}
