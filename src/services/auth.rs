use crate::models::auth::Claims;
use crate::models::user::{User, UserInfo};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::{debug, error, info, warn};
use sqlx::PgPool;
use sqlx::Row;
use std::env;

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

pub fn verify_jwt(
    token: &str,
) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    debug!("Looking up user with email: {}", email);

    let row = sqlx::query(
        "SELECT id, email, company_id, full_name, initial, created_at, updated_at 
         FROM users 
         WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => {
            let user = User {
                id: row.try_get("id")?,
                email: row.try_get("email")?,
                company_id: row.try_get("company_id")?,
                full_name: row.try_get("full_name")?,
                initial: row.try_get("initial")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            };

            info!("User found for email: {}", email);
            Ok(Some(user))
        }
        None => {
            warn!("No user found for email: {}", email);
            Ok(None)
        }
    }
}
