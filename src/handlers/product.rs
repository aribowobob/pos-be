// file: /Users/catalyst/Documents/playground/pos-be/src/handlers/product.rs
use crate::models::{AppState, response::ApiResponse};
use crate::models::product::{ProductCategoryQueryParams, ProductQueryParams, NewProduct};
use crate::services::db_service::DbConnectionManager;
use crate::services::product_service;
use actix_web::{web, HttpResponse, HttpRequest};
use log::{error, info};

pub async fn get_product_categories(
    req: HttpRequest,
    query: web::Query<ProductCategoryQueryParams>,
    data: web::Data<AppState>,
) -> HttpResponse {
    info!("Processing get_product_categories request");
    
    // Create database connection manager
    let db_manager = DbConnectionManager::new(data.db_connection_string.clone());
    
    // Extract authentication using our helper function
    let auth_result = crate::middleware::extract_auth::extract_auth_user(&req, &db_manager).await;
    
    // Handle authentication result
    let (_user, _company_id) = match auth_result {
        Ok((user, company_id)) => {
            info!("User authenticated: {} (company_id: {})", user.email, company_id);
            (user, company_id)
        },
        Err(e) => {
            error!("Authentication failed: {:?}", e);
            return HttpResponse::Unauthorized().json(ApiResponse {
                status: "error".to_string(),
                message: format!("Authentication failed: {}", e),
                data: None::<()>,
            });
        }
    };
    
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
    req: HttpRequest,
    data: web::Data<AppState>,
    product_data: web::Json<NewProduct>,
) -> HttpResponse {
    info!("Processing create_product request");
    
    // Create database connection manager
    let db_manager = DbConnectionManager::new(data.db_connection_string.clone());
    
    // Extract authentication using our helper function
    let auth_result = crate::middleware::extract_auth::extract_auth_user(&req, &db_manager).await;
    
    // Handle authentication result
    let (_user, company_id) = match auth_result {
        Ok((user, company_id)) => {
            info!("User authenticated for create_product: {} (company_id: {})", user.email, company_id);
            (user, company_id)
        },
        Err(e) => {
            error!("Authentication failed for create_product: {:?}", e);
            return HttpResponse::Unauthorized().json(ApiResponse {
                status: "error".to_string(),
                message: format!("Authentication failed: {}", e),
                data: None::<()>,
            });
        }
    };
    
    // Create product with company_id from user
    let product = product_data.into_inner();
    
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

pub async fn get_products(
    req: HttpRequest,
    query: web::Query<ProductQueryParams>,
    data: web::Data<AppState>,
) -> HttpResponse {
    info!("Processing get_products request");
    
    // Create database connection manager
    let db_manager = DbConnectionManager::new(data.db_connection_string.clone());
    
    // Extract authentication using our helper function
    let auth_result = crate::middleware::extract_auth::extract_auth_user(&req, &db_manager).await;
    
    // Handle authentication result
    let (_user, company_id) = match auth_result {
        Ok((user, company_id)) => {
            info!("User authenticated for get_products: {} (company_id: {})", user.email, company_id);
            (user, company_id)
        },
        Err(e) => {
            error!("Authentication failed for get_products: {:?}", e);
            return HttpResponse::Unauthorized().json(ApiResponse {
                status: "error".to_string(),
                message: format!("Authentication failed: {}", e),
                data: None::<()>,
            });
        }
    };
    
    // Call the service to get products
    match product_service::get_products(
        &db_manager,
        company_id,
        query.search.clone(),
        query.page,
        query.size,
    ).await {
        Ok(products) => {
            info!("Successfully retrieved products for company_id {}", company_id);
            HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "Products retrieved successfully".to_string(),
                data: Some(products),
            })
        }
        Err(e) => {
            error!("Failed to retrieve products: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: format!("Failed to retrieve products: {:?}", e),
                data: None::<()>,
            })
        }
    }
}
