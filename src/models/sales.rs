use chrono::{DateTime, Utc, NaiveDate};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSalesCart {
    pub base_price: Option<Decimal>,
    pub qty: Option<i32>,
    pub discount_type: Option<String>,
    pub discount_value: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SalesOrder {
    pub id: i32,
    pub order_number: String,
    pub user_id: i32,
    pub store_id: i32,
    pub date: NaiveDate,
    pub grand_total: Decimal,
    pub payment_cash: Decimal,
    pub payment_non_cash: Decimal,
    pub receivable: Decimal,
    pub created_at: DateTime<Utc>,
    pub customer_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SalesOrderDetail {
    pub id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub qty: i32,
    pub base_price: Decimal,
    pub discount_type: String,
    pub discount_value: Decimal,
    pub discount_amount: Decimal, 
    pub sale_price: Decimal,
    pub total_price: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub order_number: String,
    pub store_id: i32,
    pub date: Option<NaiveDate>,
    pub payment_cash: Decimal,
    pub payment_non_cash: Decimal,
    pub customer_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order: SalesOrder,
    pub details: Vec<SalesOrderDetail>,
}
