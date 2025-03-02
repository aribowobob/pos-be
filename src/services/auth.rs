use crate::models::auth::{Claims};
use crate::models::user::UserInfo;
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;

pub async fn verify_google_token(token: &str) -> Result<UserInfo, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    
    let response_text = response.text().await?;
    let user_info: UserInfo = serde_json::from_str(&response_text)?;
    Ok(user_info)
}

pub fn create_jwt(user_info: &UserInfo) -> String {
    let claims = Claims {
        sub: user_info.sub.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        email: user_info.email.clone().unwrap_or_default(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes())
    ).unwrap()
}
