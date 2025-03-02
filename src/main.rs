use actix_web::{App, HttpServer, HttpResponse, web, get};
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
