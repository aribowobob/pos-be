mod handlers;
mod models;
mod services;

use actix_web::{App, HttpServer, HttpResponse, get};
use dotenv::dotenv;

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Hello, welcome to POS Backend!"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    println!("Server starting at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(handlers::auth::google_auth)
            .wrap(actix_cors::Cors::permissive())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
