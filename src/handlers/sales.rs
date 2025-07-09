use crate::models::{AppState, response::ApiResponse};
use crate::models::sales::{NewSalesCart, UpdateSalesCart, CreateOrderRequest, SalesReport, DetailedOrderResponse, SalesReportQuery};
use crate::services::db_service::DbConnectionManager;
use crate::services::sales_service;
use actix_web::{web, HttpResponse, HttpRequest};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetCartQuery {
    /// Store ID to get cart items for
    pub store_id: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ClearCartQuery {
    /// Store ID to clear the cart for
    pub store_id: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetSalesReportQuery {
    /// Start date for the report (YYYY-MM-DD)
    pub start_date: chrono::NaiveDate,
    /// End date for the report (YYYY-MM-DD)
    pub end_date: chrono::NaiveDate,
    /// Store ID (0 for all stores)
    pub store_id: i32,
}
use log::{error, info};

#[utoipa::path(
    post,
    path = "/api/sales/cart",
    request_body = NewSalesCart,
    responses(
        (status = 201, description = "Item added to cart successfully", body = ApiResponse<SalesCart>),
        (status = 401, description = "Authentication required", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    security(
        ("cookie_auth" = [])
    ),
    tag = "sales"
)]
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

#[utoipa::path(
    delete,
    path = "/api/sales/cart/{id}",
    params(
        ("id" = i32, Path, description = "Cart item ID to delete")
    ),
    responses(
        (status = 200, description = "Item deleted successfully", body = ApiResponse<String>),
        (status = 401, description = "Authentication required", body = ApiResponse<()>),
        (status = 404, description = "Item not found or not owned by user", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    security(
        ("cookie_auth" = [])
    ),
    tag = "sales"
)]
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

#[utoipa::path(
    get,
    path = "/api/sales/cart",
    params(
        GetCartQuery
    ),
    responses(
        (status = 200, description = "Cart items retrieved successfully", body = ApiResponse<Vec<SalesCart>>),
        (status = 401, description = "Authentication required", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    security(
        ("cookie_auth" = [])
    ),
    tag = "sales"
)]
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

#[utoipa::path(
    put,
    path = "/api/sales/cart/{id}",
    params(
        ("id" = i32, Path, description = "Cart item ID to update")
    ),
    request_body = UpdateSalesCart,
    responses(
        (status = 200, description = "Item updated successfully", body = ApiResponse<SalesCart>),
        (status = 401, description = "Authentication required", body = ApiResponse<()>),
        (status = 404, description = "Item not found or not owned by user", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    security(
        ("cookie_auth" = [])
    ),
    tag = "sales"
)]
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

#[utoipa::path(
    post,
    path = "/api/sales/orders",
    request_body = CreateOrderRequest,
    responses(
        (status = 201, description = "Order created successfully", body = ApiResponse<i32>),
        (status = 401, description = "Authentication required", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    security(
        ("cookie_auth" = [])
    ),
    tag = "sales"
)]
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

// These query parameter structs are now defined at the top of the file

#[utoipa::path(
    delete,
    path = "/api/sales/cart/clear",
    params(
        ClearCartQuery
    ),
    responses(
        (status = 200, description = "Cart cleared successfully", body = ApiResponse<String>),
        (status = 401, description = "Authentication required", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    security(
        ("cookie_auth" = [])
    ),
    tag = "sales"
)]
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

// Query parameters structs are defined at the top of the file

#[utoipa::path(
    get,
    path = "/api/sales/report",
    params(
        GetSalesReportQuery
    ),
    responses(
        (status = 200, description = "Sales report generated successfully", body = ApiResponse<SalesReport>),
        (status = 401, description = "Authentication required", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    security(
        ("cookie_auth" = [])
    ),
    tag = "sales"
)]
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

#[utoipa::path(
    get,
    path = "/api/sales/orders/{id}",
    params(
        ("id" = i32, Path, description = "Sales order ID to retrieve")
    ),
    responses(
        (status = 200, description = "Sales order retrieved successfully", body = ApiResponse<DetailedOrderResponse>),
        (status = 401, description = "Authentication required", body = ApiResponse<()>),
        (status = 404, description = "Order not found", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    security(
        ("cookie_auth" = [])
    ),
    tag = "sales"
)]
pub async fn get_sales_order_by_id(
    req: HttpRequest,
    data: web::Data<AppState>,
    path: web::Path<(i32,)>, // Sales order ID from path
) -> HttpResponse {
    let order_id = path.0;
    info!("Processing get_sales_order_by_id request for order ID: {}", order_id);
    
    // Create database connection manager
    let db_manager = DbConnectionManager::new(data.db_connection_string.clone());
    
    // Extract authentication using our helper function
    let auth_result = crate::middleware::extract_auth::extract_auth_user(&req, &db_manager).await;
    
    // Handle authentication result
    let user = match auth_result {
        Ok((user, _)) => {
            info!("User authenticated: {}", user.email);
            user
        },
        Err(e) => {
            error!("Authentication failed: {:?}", e);
            return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(&format!("Authentication failed: {}", e)));
        }
    };
    
    // Process the request with the authenticated user's ID, simplified to just require login
    match sales_service::get_sales_order_by_id(&db_manager, order_id, user.id).await {
        Ok(order_response) => {
            info!("Successfully retrieved sales order ID: {}", order_id);
            HttpResponse::Ok().json(ApiResponse::success(order_response))
        },
        Err(e) => {
            match e {
                crate::errors::ServiceError::NotFound => {
                    error!("Sales order not found or not accessible: {:?}", e);
                    HttpResponse::NotFound().json(ApiResponse::<()>::error("Sales order not found or not accessible by this user"))
                },
                _ => {
                    error!("Failed to retrieve sales order: {:?}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&format!("Failed to retrieve sales order: {}", e)))
                }
            }
        }
    }
}
