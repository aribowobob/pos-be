mod handlers;
mod models;
mod services;

use actix_web::{App, HttpServer, HttpResponse, get, http::header};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Hello, welcome to POS Backend!"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    
    println!("Server starting at http://127.0.0.1:8080");
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(hello)
            .service(handlers::auth::google_auth)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
