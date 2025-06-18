use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub company_id: i32,
    pub full_name: String,
    pub initial: String,
    pub company_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Store {
    pub id: i32,
    pub name: String,
    pub company_id: i32,
    pub initial: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserWithStores {
    pub id: i32,
    pub email: String,
    pub company_id: i32,
    pub company_name: Option<String>,
    pub full_name: String,
    pub initial: String,
    pub stores: Vec<Store>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]  // Allow unused fields as they are used indirectly during token verification
pub struct UserInfo {
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub sub: String,
    #[serde(skip)]
    pub id: i32,
    #[serde(skip)]
    pub full_name: String,
    #[serde(skip)]
    pub company_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub company_id: i32,
    pub full_name: String,
    pub initial: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserStore {
    pub user_id: i32,
    pub store_id: i32,
}
