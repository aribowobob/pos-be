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
    #[serde(flatten)]
    pub user: User,
    pub stores: Vec<Store>,
}

impl UserWithStores {
    pub fn from_user_and_stores(user: User, stores: Vec<Store>) -> Self {
        Self { user, stores }
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
