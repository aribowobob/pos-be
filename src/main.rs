mod errors;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;

use crate::models::AppState;
use actix_cors::Cors;
use actix_web::{
    get, http::header, middleware as actix_middleware, web, App, HttpResponse, HttpServer,
};
use dotenv::dotenv;
use env_logger::{Builder, Env};
use log::{info, LevelFilter};
use middleware::auth::AuthMiddleware;
use std::env;
use std::io;

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Hello, welcome to POS Backend!"
    }))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    // Initialize logger with more detailed configuration
    Builder::new()
        .filter_level(LevelFilter::Info)
        .format_timestamp_millis()
        .format_target(true)
        .parse_env(Env::default().default_filter_or("info"))
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Configure multiple allowed origins
    let frontend_urls: Vec<String> = env::var("FRONTEND_URLS")
        .unwrap_or_else(|_| {
            // Default: allow both localhost and the production URL
            "http://localhost:3000,https://your-production-url.com,https://staging-url.com".to_string()
        })
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    info!("Database connection will be established on-demand");
    info!("Allowed CORS origins: {:?}", frontend_urls);

    // Get port from environment variable or use default 8080
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    info!("Server starting at http://{}", addr);

    HttpServer::new(move || {
        // Initialize CORS configuration with proper headers for cookie support
        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS", "PATCH"])
            .allowed_headers(vec![
                header::AUTHORIZATION, 
                header::CONTENT_TYPE,
                header::ACCEPT,
                header::ORIGIN,
                header::SET_COOKIE,
                header::COOKIE,
                header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                header::ACCESS_CONTROL_REQUEST_HEADERS,
                header::ACCESS_CONTROL_EXPOSE_HEADERS,
            ])
            .expose_headers(vec![
                "content-length",
                "Set-Cookie",
                "Authorization"
            ])
            .supports_credentials() // This enables cookies, authorization headers and TLS certificates
            .max_age(3600);
            
        // Add each allowed origin to CORS configuration
        for origin in &frontend_urls {
            info!("Adding allowed origin: {}", origin);
            cors = cors.allowed_origin(origin);
        }

        App::new()
            .app_data(web::Data::new(AppState {
                db_connection_string: database_url.clone(),
            }))
            .wrap(cors)
            .wrap(AuthMiddleware)
            .wrap(actix_middleware::Logger::default())
            .configure(routes::configure_routes)
    })
    .bind(&addr)?
    .run()
    .await
}
