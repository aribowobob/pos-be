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
