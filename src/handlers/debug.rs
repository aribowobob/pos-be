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
    
    // Basic connection string analysis without exposing password
    let db_url = &app_state.db_connection_string;
    let mut debug_info = HashMap::new();
    
    // Extract basic info from connection string safely
    if db_url.starts_with("postgres://") {
        debug_info.insert("scheme".to_string(), "postgres".to_string());
        
        // Try to extract host without exposing password
        if let Some(at_pos) = db_url.find('@') {
            if let Some(colon_pos) = db_url[at_pos..].find(':') {
                let host_start = at_pos + 1;
                let host_end = at_pos + colon_pos;
                if host_end > host_start && host_end < db_url.len() {
                    let host = &db_url[host_start..host_end];
                    debug_info.insert("host".to_string(), host.to_string());
                }
            }
        }
        
        // Check if it contains placeholder variables (not substituted)
        if db_url.contains("${") {
            debug_info.insert("env_substitution".to_string(), "variables_not_substituted".to_string());
        } else {
            debug_info.insert("env_substitution".to_string(), "appears_substituted".to_string());
        }
    } else {
        debug_info.insert("scheme".to_string(), "unknown".to_string());
    }
    
    debug_info.insert("connection_string_length".to_string(), db_url.len().to_string());
    
    // Add partial connection string for debugging (first and last 10 chars, hide middle)
    if db_url.len() > 20 {
        let start = &db_url[..10];
        let end = &db_url[db_url.len()-10..];
        debug_info.insert("connection_string_preview".to_string(), format!("{}...{}", start, end));
    } else {
        debug_info.insert("connection_string_preview".to_string(), "too_short".to_string());
    }
    
    match db_service::get_db_pool(&app_state.db_connection_string).await {
        Ok(pool) => {
            // Try to execute a simple query
            match sqlx::query("SELECT 1 as test_value").fetch_one(&pool).await {
                Ok(_) => {
                    info!("Database connection test successful");
                    debug_info.insert("connection_status".to_string(), "success".to_string());
                    debug_info.insert("query_test".to_string(), "passed".to_string());
                    
                    HttpResponse::Ok().json(ApiResponse {
                        status: "success".to_string(),
                        message: "Database connection successful".to_string(),
                        data: Some(debug_info),
                    })
                },
                Err(e) => {
                    info!("Database query failed: {}", e);
                    debug_info.insert("connection_status".to_string(), "connected".to_string());
                    debug_info.insert("query_test".to_string(), "failed".to_string());
                    debug_info.insert("query_error".to_string(), e.to_string());
                    
                    HttpResponse::InternalServerError().json(ApiResponse {
                        status: "error".to_string(),
                        message: format!("Database query failed: {}", e),
                        data: Some(debug_info),
                    })
                }
            }
        },
        Err(e) => {
            info!("Database connection failed: {:?}", e);
            debug_info.insert("connection_status".to_string(), "failed".to_string());
            debug_info.insert("connection_error".to_string(), format!("{:?}", e));
            
            HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: format!("Database connection failed: {:?}", e),
                data: Some(debug_info),
            })
        }
    }
}
