use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SalesCart {
    pub id: i32,
    pub user_id: i32,
    pub store_id: i32,
    pub product_id: i32,
    pub base_price: Decimal,
    pub qty: i32,
    pub discount_type: String,
    pub discount_value: i32,
    pub discount_amount: Decimal,
    pub sale_price: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewSalesCart {
    pub store_id: i32,
    pub product_id: i32,
    pub base_price: Decimal,
    pub qty: i32,
    pub discount_type: Option<String>,
    pub discount_value: Option<i32>,
    pub discount_amount: Option<Decimal>,
    pub sale_price: Option<Decimal>,
}
