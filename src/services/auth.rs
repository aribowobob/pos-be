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

pub async fn verify_google_token(token: &str) -> Result<UserInfo, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    match client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token)
        .send()
        .await
    {
        Ok(response) => {
            if !response.status().is_success() {
                let error_text = response.text().await?;
                error!("Google API error: {}", error_text);
                return Err(format!("Failed to verify token: {error_text}").into());
            }

            let response_text = response.text().await?;
            info!("Google authentication successful");
            debug!("Google API response: {}", response_text);

            match serde_json::from_str::<UserInfo>(&response_text) {
                Ok(mut user_info) => {
                    debug!(
                        "Successfully parsed user info for email: {}",
                        user_info.email
                    );
                    // Set default values for fields that are not from Google
                    user_info.id = 0; // This will be populated from the database when found
                    user_info.full_name = user_info.name.clone().unwrap_or_default();
                    user_info.company_id = 0; // This will be populated from the database when found

                    Ok(user_info)
                }
                Err(e) => {
                    error!("Failed to parse user info: {}", e);
                    Err(Box::new(e))
                }
            }
        }
        Err(e) => {
            error!("Request to Google API failed: {}", e);
            Err(Box::new(e))
        }
    }
}

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
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| ServiceError::Unauthorized)
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
