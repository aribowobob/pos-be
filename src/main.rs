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
    let frontend_url =
        env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    info!("Server starting at http://127.0.0.1:8080");
    info!("Database connection will be established on-demand");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(AppState {
                db_connection_string: database_url.clone(),
            }))
            .wrap(cors)
            .wrap(AuthMiddleware)
            .wrap(actix_middleware::Logger::default())
            .configure(routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
