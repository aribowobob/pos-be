use crate::models::{AppState, response::ApiResponse};
use crate::models::product::{ProductCategoryQueryParams, ProductQueryParams, NewProduct};
use crate::services::db_service::DbConnectionManager;
use crate::services::product_service;
use crate::errors::ServiceError;
use actix_web::{web, HttpResponse, HttpRequest};
use log::{error, info};

// Get product categories
// Returns a list of product categories with optional filtering and pagination
#[utoipa::path(
    get,
    path = "/products/categories",
    params(
        ProductCategoryQueryParams
    ),
    responses(
        (status = 200, description = "Product categories retrieved successfully", body = ApiResponse<Vec<ProductCategory>>),
        (status = 401, description = "Authentication required", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    security(
        ("cookie_auth" = [])
    ),
    tag = "products"
)]
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

// Create new product
// Adds a new product to the database
#[utoipa::path(
    post,
    path = "/products",
    request_body = NewProduct,
    responses(
        (status = 201, description = "Product created successfully", body = ApiResponse<Product>),
        (status = 400, description = "Invalid product data", body = ApiResponse<()>),
        (status = 401, description = "Authentication required", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    security(
        ("cookie_auth" = [])
    ),
    tag = "products"
)]
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

#[utoipa::path(
    get,
    path = "/products",
    params(
        ProductQueryParams
    ),
    responses(
        (status = 200, description = "Products retrieved successfully", body = ApiResponse<Vec<Product>>),
        (status = 401, description = "Authentication required", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    security(
        ("cookie_auth" = [])
    ),
    tag = "products"
)]
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

#[utoipa::path(
    get,
    path = "/products/{id}",
    params(
        ("id" = i32, Path, description = "Product ID to retrieve")
    ),
    responses(
        (status = 200, description = "Product retrieved successfully", body = ApiResponse<Product>),
        (status = 401, description = "Authentication required", body = ApiResponse<()>),
        (status = 404, description = "Product not found", body = ApiResponse<()>),
        (status = 500, description = "Internal server error", body = ApiResponse<()>)
    ),
    security(
        ("cookie_auth" = [])
    ),
    tag = "products"
)]
pub async fn get_product_by_id(
    req: HttpRequest,
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let product_id = path.into_inner();
    info!("Processing get_product_by_id request for product_id: {}", product_id);
    
    // Create database connection manager
    let db_manager = DbConnectionManager::new(data.db_connection_string.clone());
    
    // Extract authentication using our helper function
    let auth_result = crate::middleware::extract_auth::extract_auth_user(&req, &db_manager).await;
    
    // Handle authentication result
    let (_user, company_id) = match auth_result {
        Ok((user, company_id)) => {
            info!("User authenticated for get_product_by_id: {} (company_id: {})", user.email, company_id);
            (user, company_id)
        },
        Err(e) => {
            error!("Authentication failed for get_product_by_id: {:?}", e);
            return HttpResponse::Unauthorized().json(ApiResponse {
                status: "error".to_string(),
                message: format!("Authentication failed: {}", e),
                data: None::<()>,
            });
        }
    };
    
    // Call the service to get product by ID, ensuring company_id match
    match product_service::get_product_by_id(&db_manager, product_id, company_id).await {
        Ok(product) => {
            info!("Successfully retrieved product with ID {} for company_id {}", product_id, company_id);
            HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "Product retrieved successfully".to_string(),
                data: Some(product),
            })
        },
        Err(ServiceError::NotFound) => {
            info!("Product with ID {} not found or not accessible for company_id {}", product_id, company_id);
            HttpResponse::NotFound().json(ApiResponse {
                status: "error".to_string(),
                message: "Product not found".to_string(),
                data: None::<()>,
            })
        },
        Err(e) => {
            error!("Failed to retrieve product by ID: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: format!("Failed to retrieve product: {:?}", e),
                data: None::<()>,
            })
        }
    }
}
