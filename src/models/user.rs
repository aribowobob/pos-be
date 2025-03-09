use crate::db::schema::{stores, user_stores, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub company_id: i32,
    pub full_name: String,
    pub initial: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = stores)]
pub struct Store {
    pub id: i32,
    pub name: String,
    pub company_id: i32,
    pub initial: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Store))]
#[diesel(table_name = user_stores)]
#[diesel(primary_key(user_id, store_id))]
pub struct UserStore {
    pub user_id: i32,
    pub store_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Response struct for the /get-user endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct UserWithStores {
    pub id: i32,
    pub email: String,
    pub company_id: i32,
    pub full_name: String,
    pub initial: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub stores: Vec<Store>,
}

// Modified UserInfo struct to match Google OAuth response
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub sub: String, // Google's unique identifier for the user
    #[serde(skip)] // Fields below are not from Google, but set by our application
    pub id: i32,
    #[serde(skip)]
    pub full_name: String,
    #[serde(skip)]
    pub company_id: i32,
}

#[allow(dead_code)]
impl UserWithStores {
    pub fn from_user_and_stores(user: User, stores: Vec<Store>) -> Self {
        Self {
            id: user.id,
            email: user.email,
            company_id: user.company_id,
            full_name: user.full_name,
            initial: user.initial,
            created_at: user.created_at,
            updated_at: user.updated_at,
            stores,
        }
    }
}

// For creating a new user
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub company_id: i32,
    pub full_name: String,
    pub initial: String,
}

// For creating a user-store association
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = user_stores)]
pub struct NewUserStore {
    pub user_id: i32,
    pub store_id: i32,
}
