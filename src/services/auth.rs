use crate::errors::ServiceError;
use crate::models::auth::Claims;
use crate::models::user::{User, UserInfo};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use log::{debug, error, info, warn};
use sqlx::PgPool;
use sqlx::Row;
use std::env;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDateTime};

// Function moved to google_auth.rs
// This function is kept here as a comment for historical reference
/*
pub async fn verify_google_token(token: &str) -> Result<UserInfo, Box<dyn std::error::Error>> {
    // Implementation moved to crate::services::google_auth::verify_google_token
}
*/

#[must_use]
pub fn create_jwt(user_info: &UserInfo) -> String {
    let claims = Claims {
        sub: user_info.sub.clone(),
        exp: usize::try_from((chrono::Utc::now() + chrono::Duration::hours(24)).timestamp())
            .unwrap_or(0),
        email: user_info.email.clone(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()),
    )
    .unwrap()
}

pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>, ServiceError> {
    let jwt_secret = match std::env::var("JWT_SECRET") {
        Ok(secret) => {
            debug!("JWT_SECRET found in environment");
            secret
        },
        Err(_) => {
            warn!("JWT_SECRET not found in environment, using default secret");
            "default_secret".to_string()
        }
    };
    
    debug!("Attempting to verify JWT token");
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| {
        error!("JWT verification failed: {:?}", e);
        ServiceError::Unauthorized
    })
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    debug!("Looking up user with email: {}", email);

    let row = sqlx::query(
        "SELECT u.id, u.email, u.company_id, u.full_name, u.initial, u.created_at, u.updated_at, c.name as company_name
         FROM users u
         LEFT JOIN companies c ON u.company_id = c.id
         WHERE u.email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => {
            debug!("Found user row, extracting data");
            
            let user = User {
                id: row.try_get::<i32, _>("id")?,
                email: row.try_get("email")?,
                company_id: row.try_get("company_id")?,
                company_name: row.try_get("company_name")?,
                full_name: row.try_get("full_name")?,
                initial: row.try_get("initial")?,
            };

            info!("User found for email: {} with ID: {}", email, user.id);
            Ok(Some(user))
        }
        None => {
            warn!("No user found for email: {}", email);
            Ok(None)
        }
    }
}
