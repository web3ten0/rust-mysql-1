use actix_web::{get, App, Responder, HttpResponse, HttpServer};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
      .bind("0.0.0.0:8080")?
      .run()
      .await
}