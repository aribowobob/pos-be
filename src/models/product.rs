// file: /Users/catalyst/Documents/playground/pos-be/src/models/product.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use utoipa::{ToSchema, IntoParams};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ProductCategory {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ProductCategoryQueryParams {
    /// Optional search term to filter categories
    pub search: Option<String>,
    /// Page number for pagination
    #[schema(default = "1")]
    pub page: Option<i32>,
    /// Number of items per page
    #[schema(default = "10")]
    pub size: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Product {
    pub id: i32,
    pub sku: String,
    pub name: String,
    pub purchase_price: Decimal,
    pub sale_price: Decimal,
    pub company_id: i32,
    pub unit_name: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub category_id: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NewProduct {
    pub sku: String,
    pub name: String,
    pub purchase_price: Decimal,
    pub sale_price: Decimal,
    pub unit_name: Option<String>,
    pub category_id: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ProductQueryParams {
    /// Optional search term to filter products
    pub search: Option<String>,
    /// Page number for pagination
    #[schema(default = "1")]
    pub page: Option<i32>,
    /// Number of items per page
    #[schema(default = "10")]
    pub size: Option<i32>,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub page: i32,
    pub size: i32,
    pub total: i64,
    pub total_pages: i64,
    pub items: Vec<T>,
}

impl<T> PaginatedResponse<T> {
    pub fn new(page: i32, size: i32, total: i64, items: Vec<T>) -> Self {
        let total_pages = if size > 0 { (total + size as i64 - 1) / size as i64 } else { 0 };
        Self {
            page,
            size,
            total,
            total_pages,
            items,
        }
    }
}
