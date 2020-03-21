use std::sync::Mutex;

use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use harsh::Harsh;

use crate::errors;
use crate::executors::PerlExecutor;
use crate::models::InstanceRequest;
use crate::services;
use crate::AppState;

pub fn routings(app: &mut web::ServiceConfig) {
  app.service(get_instance).service(create_instance);
}

#[actix_web::get("/instances/{instance_id}")]
async fn get_instance(
  _: HttpRequest,
  state: web::Data<AppState>,
  harsh: web::Data<Harsh>,
  instance_id: web::Path<String>,
) -> Result<impl Responder, Error> {
  let instance_id: u64 = match harsh.decode(&instance_id.into_inner()) {
    Some(value) => value[0],
    None => return Err(errors::ServerError::InternalServerError.into()),
  };

  let db = state.db.clone();
  let instance = services::instance::fetch_instance(db, instance_id as i64).await?;

  Ok(HttpResponse::Ok().json(instance))
}

#[actix_web::post("/instances")]
async fn create_instance(
  _: HttpRequest,
  state: web::Data<AppState>,
  executor: web::Data<Mutex<PerlExecutor>>,
  data: web::Json<InstanceRequest>,
) -> Result<impl Responder, Error> {
  let db = state.db.clone();

  let (instance_id, instance) = services::instance::create_instance(db, data.into_inner()).await?;

  let rx = executor.lock().unwrap().execute(instance_id, instance);

  Ok(
    HttpResponse::Ok()
      .header("Content-Type", "text/event-stream")
      .no_chunking()
      .streaming(rx),
  )
}

/*
#[derive(Clone, Debug, Serialize)]
pub struct InstanceMinimalResponse {
  instance_id: String,
}
*/

/*
#[actix_web::post("/instances")]
async fn create_instance(
  _: HttpRequest,
  db: web::Data<database::DbPool>,
  harsh: web::Data<Harsh>,
  runner: web::Data<Mutex<services::instance_runner::InstanceRunner>>,
  payload: web::Json<InstanceJson>,
) -> Result<impl Responder, errors::ServerError> {
  let connection = database::extract_connection(db)?;
  let payload = payload.into_inner();

  let instance =
    web::block(move || services::instance::create_instance(&connection, payload.clone()))
      .await
      .map_err(|_| errors::ServerError::InternalServerError)?;

  if instance.is_none() {
    return Err(errors::ServerError::InternalServerError);
  }

  let instance = instance.unwrap();

  let instance_id = match harsh.encode(&[instance.id as u64]) {
    Some(value) => value,
    None => return Err(errors::ServerError::InternalServerError),
  };

  let rx = runner.lock().unwrap().new_runner(&instance_id, instance);

  Ok(
    HttpResponse::Ok()
      .header("Content-Type", "text/event-stream")
      .no_chunking()
      .streaming(rx),
  )
}
*/
