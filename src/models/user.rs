use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub company_id: i32,
    pub full_name: String,
    pub initial: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub company_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Store {
    pub id: i32,
    pub name: String,
    pub company_id: i32,
    pub initial: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct UserWithStores {
    pub id: i32,
    pub email: String,
    pub company_id: i32,
    pub company_name: Option<String>, // Add company name field
    pub full_name: String,
    pub initial: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub stores: Vec<Store>,
}

impl UserWithStores {
    pub fn from_user_and_stores(user: User, stores: Vec<Store>) -> Self {
        Self {
            id: user.id,
            email: user.email,
            company_id: user.company_id,
            company_name: user.company_name, // Include company name here
            full_name: user.full_name,
            initial: user.initial,
            created_at: user.created_at,
            updated_at: user.updated_at,
            stores,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
