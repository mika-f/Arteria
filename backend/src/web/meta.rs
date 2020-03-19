use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize)]
struct ArteriaVersion<'a> {
  version: &'a str,
}

pub fn routings(app: &mut web::ServiceConfig) {
  app.service(version);
}

#[actix_web::get("/meta/version")]
async fn version(_: HttpRequest) -> impl Responder {
  HttpResponse::Ok().json(ArteriaVersion { version: VERSION })
}
