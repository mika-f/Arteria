use actix_web::{web, HttpRequest, HttpResponse, Responder};
use harsh::Harsh;

use crate::database;
use crate::errors;
use crate::services;

pub fn routings(app: &mut web::ServiceConfig) {
  app.service(get_instance);
}

#[actix_web::get("/instances/{instance_id}")]
async fn get_instance(
  _: HttpRequest,
  db: web::Data<database::DbPool>,
  harsh: web::Data<Harsh>,
  instance_id: web::Path<String>,
) -> Result<impl Responder, errors::ServerError> {
  let connection = database::extract_connection(db)?;

  let instance_id: u64 = match harsh.decode(&instance_id.into_inner()) {
    Some(value) => value[0],
    None => return Err(errors::ServerError::InternalServerError),
  };

  let instance = web::block(move || services::instance::fetch_instance(&connection, instance_id))
    .await
    .map_err(|_| errors::ServerError::InternalServerError)?;

  if instance.is_none() {
    return Err(errors::ServerError::ResourceNotFound);
  }

  Ok(HttpResponse::Ok().json(instance.unwrap()))
}
