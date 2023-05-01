use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn hallo_index() -> impl Responder {
    HttpResponse::Ok().body("hallo duniaaaaaa!")
}

async fn get_items() -> impl Responder {
    HttpResponse::Ok().body("list of items")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(hallo_index)
            .route("/items", web::get().to(get_items))
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
