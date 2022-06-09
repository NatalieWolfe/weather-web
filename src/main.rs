use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index(_params: web::Path<()>) -> impl Responder {
  format!("Hello!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().service(index))
    .bind(("::0", 8080))?
    .run()
    .await
}
