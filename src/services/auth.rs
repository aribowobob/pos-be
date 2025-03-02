use crate::models::auth::{Claims, GoogleTokenInfo};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;

pub async fn verify_google_token(token: &str) -> Result<GoogleTokenInfo, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .get("https://oauth2.googleapis.com/tokeninfo")
        .query(&[("id_token", token)])
        .send()
        .await?
        .json()
        .await
}

pub fn create_jwt(user_info: &GoogleTokenInfo) -> String {
    let claims = Claims {
        sub: user_info.email.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        email: user_info.email.clone(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes())
    ).unwrap()
}
