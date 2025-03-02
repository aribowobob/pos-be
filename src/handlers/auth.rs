use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use crate::services::auth::{verify_google_token, create_jwt};

#[derive(Deserialize)]
pub struct TokenRequest {
    token: String,
}

#[post("/auth/google")]
pub async fn google_auth(req: web::Json<TokenRequest>) -> HttpResponse {
    match verify_google_token(&req.token).await {
        Ok(user_info) => {
            let token = create_jwt(&user_info);
            HttpResponse::Ok().json(serde_json::json!({
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
