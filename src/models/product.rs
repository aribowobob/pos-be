use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use utoipa::{ToSchema, IntoParams};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ProductCategory {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ProductCategoryQueryParams {
    // Optional search term to filter categories
    pub search: Option<String>,
    // Page number for pagination
    #[schema(default = "1")]
    pub page: Option<i32>,
    // Number of items per page
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
    pub deleted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub category_id: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NewProduct {
    #[schema(example = "SKU001")]
    pub sku: String,
    #[schema(example = "Sample Product")]
    pub name: String,
    #[schema(example = "10.50", value_type = String)]
    pub purchase_price: Decimal,
    #[schema(example = "15.99", value_type = String)]
    pub sale_price: Decimal,
    #[schema(example = "piece")]
    pub unit_name: Option<String>,
    #[schema(example = 1)]
    pub category_id: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ProductQueryParams {
    // Optional search term to filter products
    pub search: Option<String>,
    // Page number for pagination
    #[schema(default = "1")]
    pub page: Option<i32>,
    // Number of items per page
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
