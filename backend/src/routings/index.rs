use actix_web::{HttpRequest, HttpResponse, Responder};

#[actix_web::get("/")]
pub async fn index(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
