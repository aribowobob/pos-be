// file: /Users/catalyst/Documents/playground/pos-be/src/models/product.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Serialize, Deserialize)]
pub struct ProductCategory {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ProductCategoryQueryParams {
    pub search: Option<String>,
    pub page: Option<i32>,
    pub size: Option<i32>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct NewProduct {
    pub sku: String,
    pub name: String,
    pub purchase_price: Decimal,
    pub sale_price: Decimal,
    pub unit_name: Option<String>,
    pub category_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ProductQueryParams {
    pub search: Option<String>,
    pub page: Option<i32>,
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
