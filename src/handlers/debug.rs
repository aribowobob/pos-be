use actix_web::{web, HttpResponse};
use log::info;
use std::env;
use std::collections::HashMap;
use crate::models::response::ApiResponse;

pub async fn debug_env() -> HttpResponse {
    info!("Processing debug_env request");
    
    // Create a map of relevant environment variables
    let mut env_map = HashMap::new();
    
    // Check JWT_SECRET (only log indication of presence, not the actual value for security)
    match env::var("JWT_SECRET") {
        Ok(_) => { env_map.insert("JWT_SECRET", "configured"); },
        Err(_) => { env_map.insert("JWT_SECRET", "not configured"); }
    }
    
    // Other relevant environment variables
    for key in ["DATABASE_URL", "ENVIRONMENT", "FRONTEND_URLS", "PORT", "GOOGLE_CLIENT_ID"] {
        match env::var(key) {
            Ok(_val) if key == "DATABASE_URL" => {
                // Redact connection string for security
                env_map.insert(key, "configured (value hidden)");
            },
            Ok(_val) => {
                env_map.insert(key, "configured");
            },
            Err(_) => {
                env_map.insert(key, "not configured");
            }
        }
    }
    
    HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: "Environment variables status".to_string(),
        data: Some(env_map),
    })
}
