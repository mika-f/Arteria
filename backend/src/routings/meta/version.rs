use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize)]
struct ArteriaVersion {
    version: String,
}

#[actix_web::get("/meta/version")]
pub async fn version(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(ArteriaVersion {
        version: VERSION.to_string(),
    })
}
