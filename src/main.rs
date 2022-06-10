use actix_web::{get, http, middleware, web};
use actix_web::{App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde_json::json;

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> impl Responder {
  let data = json!({
    "forecasts": [
      {"name": "Redmond", "precipitation": 12.34f64},
      {"name": "Portland", "precipitation": 1.23f64}
    ]
  });
  HttpResponse::Ok()
    .content_type(http::header::ContentType::html())
    .body(hb.render("index", &data).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let mut handlebars = Handlebars::new();
  handlebars
    .register_templates_directory(".html", "./static/templates")
    .unwrap();
  let handlebars_ref = web::Data::new(handlebars);

  HttpServer::new(move || {
      App::new()
        .wrap(middleware::Compress::default())
        .wrap(middleware::Logger::default())
        .app_data(handlebars_ref.clone())
        .service(index)
    })
    .bind(("::0", 8080))?
    .run()
    .await
}
