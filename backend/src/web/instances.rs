use actix_web::{web, HttpRequest, HttpResponse, Responder};
use harsh::Harsh;
use serde::{Deserialize, Serialize};

use crate::database;
use crate::errors;
use crate::services;

pub fn routings(app: &mut web::ServiceConfig) {
  app.service(get_instance).service(create_instance);
}

#[derive(Clone, Debug, Deserialize)]
pub struct InstanceDependencyPayload {
  name_with_version: String,
}

impl services::instance::DependencyPayload for InstanceDependencyPayload {
  fn name_with_version(&self) -> &str {
    &self.name_with_version
  }
}

#[derive(Clone, Debug, Deserialize)]
pub struct InstanceFilePayload {
  title: String,
  content: String,
}

impl services::instance::FilePayload for InstanceFilePayload {
  fn title(&self) -> &str {
    &self.title
  }

  fn content(&self) -> &str {
    &self.content
  }
}

#[derive(Clone, Debug, Deserialize)]
pub struct InstancePayload {
  title: String,
  executor: String,
  dependencies: Vec<InstanceDependencyPayload>,
  files: Vec<InstanceFilePayload>,
}

impl services::instance::InstancePayload<InstanceDependencyPayload, InstanceFilePayload>
  for InstancePayload
{
  fn title(&self) -> &str {
    &self.title
  }

  fn executor_str(&self) -> &str {
    &self.executor
  }

  fn dependencies(&self) -> Vec<&InstanceDependencyPayload> {
    self.dependencies.iter().collect()
  }

  fn files(&self) -> Vec<&InstanceFilePayload> {
    self.files.iter().collect()
  }
}

#[derive(Clone, Debug, Serialize)]
pub struct InstanceMinimalResponse {
  instance_id: String,
}

#[actix_web::post("/instances")]
async fn create_instance(
  _: HttpRequest,
  db: web::Data<database::DbPool>,
  harsh: web::Data<Harsh>,
  payload: web::Json<InstancePayload>,
) -> Result<impl Responder, errors::ServerError> {
  let connection = database::extract_connection(db)?;

  let instance_id =
    web::block(move || services::instance::create_instance(&connection, payload.into_inner()))
      .await
      .map_err(|_| errors::ServerError::InternalServerError)?;

  if instance_id.is_none() {
    return Err(errors::ServerError::InternalServerError);
  }

  let instance_id = match harsh.encode(&[instance_id.unwrap() as u64]) {
    Some(value) => value,
    None => return Err(errors::ServerError::InternalServerError),
  };

  Ok(HttpResponse::Ok().json(InstanceMinimalResponse { instance_id }))
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
