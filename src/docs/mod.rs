pub mod schemas;

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::{
    models::{
        auth::TokenRequest,
        product::{ProductCategoryQueryParams, NewProduct, ProductQueryParams, ProductCategory, Product},
        response::ApiResponse,
        sales::{
            UpdateSalesCart, CreateOrderRequest, SalesReportQuery, SalesReport, SalesCart, SalesCartResponse,
            NewSalesCart, SalesSummary, DetailedOrderResponse
        },
        user::User,
    },
    handlers::sales::{
        GetCartQuery, ClearCartQuery, GetSalesReportQuery
    }
};

use self::schemas::{StringResponse, UnitResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth endpoints
        crate::handlers::auth::google_login,
        crate::handlers::auth::logout,
        
        // User endpoints
        crate::handlers::user::get_user,
        
        // Product endpoints
        crate::handlers::product::get_product_categories,
        crate::handlers::product::create_product,
        crate::handlers::product::get_products,
        crate::handlers::product::get_product_by_id,
        
        // Sales endpoints
        crate::handlers::sales::add_to_cart,
        crate::handlers::sales::delete_from_cart,
        crate::handlers::sales::get_cart_items,
        crate::handlers::sales::update_cart_item,
        crate::handlers::sales::create_order,
        crate::handlers::sales::clear_cart,
        crate::handlers::sales::get_sales_report,
        crate::handlers::sales::get_sales_order_by_id,
        
        // Migration endpoints
        crate::handlers::migration::db_migration,
    ),
    components(
        schemas(
            ApiResponse<String>,
            ApiResponse<()>,
            ApiResponse<Product>,
            ApiResponse<Vec<ProductCategory>>,
            ApiResponse<SalesCartResponse>,
            ApiResponse<Vec<SalesCartResponse>>,
            ApiResponse<User>,
            StringResponse,
            UnitResponse,
            TokenRequest,
            ProductCategoryQueryParams,
            ProductCategory,
            Product,
            NewProduct,
            ProductQueryParams,
            GetCartQuery,
            ClearCartQuery,
            NewSalesCart,
            UpdateSalesCart,
            CreateOrderRequest,
            GetSalesReportQuery,
            SalesReport,
            SalesCart,
            SalesCartResponse
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "products", description = "Product management endpoints"),
        (name = "sales", description = "Sales and cart management endpoints"),
        (name = "system", description = "System administration endpoints"),
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        );
        components.add_security_scheme(
            "cookie_auth",
            SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("access_token"))),
        );
    }
}
