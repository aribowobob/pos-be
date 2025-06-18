// Define schema examples for OpenAPI documentation
use utoipa::ToSchema;
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

// Simple string response for Swagger documentation
#[derive(Serialize, Deserialize, ToSchema)]
pub struct StringResponse {
    pub value: String
}

// Unit response - used for displaying empty responses in OpenAPI
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UnitResponse {
    _placeholder: Option<bool>
}
