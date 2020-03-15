use actix_web::{HttpRequest, HttpResponse};
use serde::Serialize;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize)]
struct ArteriaVersion {
    version: String,
}

pub async fn version(_: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(ArteriaVersion {
        version: VERSION.to_string(),
    })
}
