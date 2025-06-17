use crate::models::{AppState, response::ApiResponse};
use crate::models::sales::{NewSalesCart, UpdateSalesCart, CreateOrderRequest, SalesReport};
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

pub async fn get_cart_items(
    req: HttpRequest,
    data: web::Data<AppState>,
    query: web::Query<GetCartQuery>,
) -> HttpResponse {
    info!("Processing get_cart_items request for store_id: {}", query.store_id);
    
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
    match sales_service::get_cart_items(&db_manager, user.id, query.store_id).await {
        Ok(cart_items) => {
            info!("Retrieved {} cart items for user ID: {}", cart_items.len(), user.id);
            HttpResponse::Ok().json(ApiResponse::success(cart_items))
        },
        Err(e) => {
            error!("Failed to retrieve cart items: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&format!("Failed to retrieve cart items: {}", e)))
        }
    }
}

pub async fn update_cart_item(
    req: HttpRequest,
    data: web::Data<AppState>,
    path: web::Path<(i32,)>, // Cart item ID from path
    cart_update: web::Json<UpdateSalesCart>,
) -> HttpResponse {
    info!("Processing update_cart_item request for item ID: {}", path.0);
    
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
    
    // Process the update request with the authenticated user's ID
    match sales_service::update_cart_item(&db_manager, path.0, user.id, cart_update.into_inner()).await {
        Ok(updated_item) => {
            info!("Item updated successfully with ID: {}", path.0);
            HttpResponse::Ok().json(ApiResponse::success(updated_item))
        },
        Err(e) => {
            error!("Failed to update cart item: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&format!("Failed to update cart item: {}", e)))
        }
    }
}

pub async fn create_order(
    req: HttpRequest,
    data: web::Data<AppState>,
    order_request: web::Json<CreateOrderRequest>,
) -> HttpResponse {
    info!("Processing create_order request");
    
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
    match sales_service::create_sales_order(&db_manager, user.id, order_request.into_inner()).await {
        Ok(response) => {
            info!("Order created successfully with ID: {}", response.order.id);
            HttpResponse::Created().json(ApiResponse::success(response))
        },
        Err(e) => {
            error!("Failed to create order: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&format!("Failed to create order: {}", e)))
        }
    }
}

// Query parameters struct for get_cart_items
#[derive(serde::Deserialize)]
pub struct GetCartQuery {
    pub store_id: i32,
}

// Query parameters struct for clear_cart
#[derive(serde::Deserialize)]
pub struct ClearCartQuery {
    pub store_id: i32,
}

pub async fn clear_cart(
    req: HttpRequest,
    data: web::Data<AppState>,
    query: web::Query<ClearCartQuery>,
) -> HttpResponse {
    info!("Processing clear_cart request for store_id: {}", query.store_id);
    
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
    
    // Process the request with the authenticated user's ID and store_id
    match sales_service::clear_cart(&db_manager, user.id, query.store_id).await {
        Ok(cleared) => {
            if cleared {
                info!("Cart cleared successfully for user ID: {} in store ID: {}", user.id, query.store_id);
                HttpResponse::Ok().json(ApiResponse::success("Cart cleared successfully"))
            } else {
                info!("No cart items found for user ID: {} in store ID: {}", user.id, query.store_id);
                HttpResponse::Ok().json(ApiResponse::success("No items to clear"))
            }
        },
        Err(e) => {
            error!("Failed to clear cart: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&format!("Failed to clear cart: {}", e)))
        }
    }
}

// Query parameters struct for sales reports
#[derive(serde::Deserialize)]
pub struct GetSalesReportQuery {
    pub start_date: chrono::NaiveDate, 
    pub end_date: chrono::NaiveDate,
    pub store_id: i32, // 0 means all stores
}

pub async fn get_sales_report(
    req: HttpRequest,
    data: web::Data<AppState>,
    query: web::Query<GetSalesReportQuery>,
) -> HttpResponse {
    info!("Processing get_sales_report request from {} to {} for store_id: {}", 
          query.start_date, query.end_date, query.store_id);
    
    // Create database connection manager
    let db_manager = DbConnectionManager::new(data.db_connection_string.clone());
    
    // Extract authentication using our helper function
    let auth_result = crate::middleware::extract_auth::extract_auth_user(&req, &db_manager).await;
    
    // Handle authentication result
    let (user, company_id) = match auth_result {
        Ok((user, company_id)) => {
            info!("User authenticated: {} (company_id: {})", user.email, company_id);
            (user, company_id)
        },
        Err(e) => {
            error!("Authentication failed: {:?}", e);
            return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(&format!("Authentication failed: {}", e)));
        }
    };
    
    // Convert query to service model
    let report_query = crate::models::sales::SalesReportQuery {
        start_date: query.start_date,
        end_date: query.end_date,
        store_id: query.store_id,
    };
    
    // Process the request with the authenticated user's ID
    match sales_service::generate_sales_report(&db_manager, user.id, company_id, report_query).await {
        Ok(report) => {
            info!("Generated sales report with {} orders", report.orders.len());
            HttpResponse::Ok().json(ApiResponse::success(report))
        },
        Err(e) => {
            error!("Failed to generate sales report: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&format!("Failed to generate sales report: {}", e)))
        }
    }
}
