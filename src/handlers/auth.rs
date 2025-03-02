use actix_web::{post, web, HttpResponse};
use crate::services::auth::{verify_google_token, create_jwt};

#[post("/auth/google")]
pub async fn google_auth(token: web::Json<String>) -> HttpResponse {
    match verify_google_token(token.as_str()).await {
        Ok(user_info) => {
            let token = create_jwt(&user_info);
            HttpResponse::Ok().json(serde_json::json!({
                "token": token,
                "user": user_info
            }))
        },
        Err(_) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid token"
        }))
    }
}
