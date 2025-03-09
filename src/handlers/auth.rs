use crate::models::response::ApiResponse;
use crate::models::AppState;
use crate::services::auth::{create_jwt, get_user_by_email, verify_google_token};
use actix_web::{cookie::Cookie, web, HttpResponse};
use log::error;
use serde::Deserialize;
use std::env;

fn create_auth_cookie(token: &str) -> Cookie {
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

    let mut cookie = Cookie::build("access_token", token.to_owned())
        .path("/")
        .http_only(true)
        .max_age(actix_web::cookie::time::Duration::hours(4));

    if environment == "production" {
        cookie = cookie
            .secure(true)
            .same_site(actix_web::cookie::SameSite::Strict);
    } else {
        cookie = cookie
            .secure(true)
            .same_site(actix_web::cookie::SameSite::None);
    }

    cookie.finish()
}

#[derive(Deserialize)]
pub struct TokenRequest {
    token: String,
}

pub async fn google_auth(req: web::Json<TokenRequest>, data: web::Data<AppState>) -> HttpResponse {
    match verify_google_token(&req.token).await {
        Ok(mut user_info) => {
            // Check if user exists in database
            match get_user_by_email(&data.db, &user_info.email).await {
                Ok(Some(user)) => {
                    // Update user_info with database values
                    user_info.id = user.id;
                    user_info.full_name = user.full_name.clone();
                    user_info.company_id = user.company_id;

                    let token = create_jwt(&user_info);
                    let cookie = create_auth_cookie(&token);

                    HttpResponse::Ok().cookie(cookie).json(ApiResponse::success(
                        serde_json::json!({
                            "authorized": true,
                        }),
                    ))
                }
                Ok(None) => HttpResponse::Unauthorized().json(
                    ApiResponse::<serde_json::Value>::error("User not registered in the system"),
                ),
                Err(e) => HttpResponse::InternalServerError().json(
                    ApiResponse::<serde_json::Value>::error(&format!("Database error: {e}")),
                ),
            }
        }
        Err(e) => {
            error!("Token verification error: {:?}", e);
            HttpResponse::Unauthorized().json(ApiResponse::<serde_json::Value>::error(&format!(
                "Invalid token: {e:?}",
            )))
        }
    }
}
