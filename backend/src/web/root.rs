use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub fn routings(app: &mut web::ServiceConfig) {
  app.service(index);
}

#[actix_web::get("/")]
async fn index(_: HttpRequest) -> impl Responder {
  HttpResponse::Ok().body("It Works!")
}
