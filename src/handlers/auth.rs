use actix_web::{post, web, HttpResponse, cookie::Cookie};
use serde::Deserialize;
use std::env;
use crate::services::auth::{verify_google_token, create_jwt};

fn create_auth_cookie(token: &str) -> Cookie {
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
    
    let mut cookie = Cookie::build("access_token", token.to_owned())
        .path("/")
        .http_only(true)
        .max_age(actix_web::cookie::time::Duration::hours(4)); // 4 hours

    if environment == "production" {
        cookie = cookie
            .secure(true)
            .same_site(actix_web::cookie::SameSite::Strict);
    } else {
        cookie = cookie
            .secure(false)
            .same_site(actix_web::cookie::SameSite::None);
    }

    cookie.finish()
}

#[derive(Deserialize)]
pub struct TokenRequest {
    token: String,
}

#[post("/auth/google")]
pub async fn google_auth(req: web::Json<TokenRequest>) -> HttpResponse {
    match verify_google_token(&req.token).await {
        Ok(user_info) => {
            let token = create_jwt(&user_info);
            let cookie = create_auth_cookie(&token);

            HttpResponse::Ok()
                .cookie(cookie)
                .json(serde_json::json!({
                    "token": token,
                    "user": user_info
                }))
        },
        Err(e) => {
            HttpResponse::Unauthorized().json(serde_json::json!({
                "error": format!("Invalid token: {:?}", e)
            }))
        }
    }
}
