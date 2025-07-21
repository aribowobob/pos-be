use actix_web::{web, HttpResponse};
use log::info;
use std::env;
use std::collections::HashMap;
use crate::models::response::ApiResponse;
use crate::models::app_state::AppState;
use crate::services::db_service;

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

pub async fn debug_db_connection(app_state: web::Data<AppState>) -> HttpResponse {
    info!("Testing database connection");
    
    match db_service::get_db_pool(&app_state.db_connection_string).await {
        Ok(pool) => {
            // Try to execute a simple query
            match sqlx::query("SELECT 1").fetch_one(&pool).await {
                Ok(_) => {
                    info!("Database connection test successful");
                    HttpResponse::Ok().json(ApiResponse {
                        status: "success".to_string(),
                        message: "Database connection successful".to_string(),
                        data: Some("Connected and query executed successfully"),
                    })
                },
                Err(e) => {
                    info!("Database query failed: {}", e);
                    HttpResponse::InternalServerError().json(ApiResponse {
                        status: "error".to_string(),
                        message: format!("Database query failed: {}", e),
                        data: None::<()>,
                    })
                }
            }
        },
        Err(e) => {
            info!("Database connection failed: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: format!("Database connection failed: {:?}", e),
                data: None::<()>,
            })
        }
    }
}
