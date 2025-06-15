// file: /Users/catalyst/Documents/playground/pos-be/src/handlers/product.rs
use crate::models::{AppState, response::ApiResponse};
use crate::models::product::ProductCategoryQueryParams;
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
