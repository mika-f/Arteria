use std::iter::Iterator;
use std::path::Path;

use actix::*;
use actix_web::Error;

use crate::database::dependency::FetchDependenciesByInstanceId;
use crate::database::executor::FetchExecutorByTag;
use crate::database::file::FetchFilesByInstanceId;
use crate::database::instance::FetchInstance;
use crate::database::transaction::CreateNewInstance;
use crate::database::DbExecutor;
use crate::errors;
use crate::models::{Executor, InstanceRequest, InstanceResponse};

pub async fn create_instance(
  database: Addr<DbExecutor>,
  object: InstanceRequest,
) -> Result<(i64, InstanceResponse, Executor), Error> {
  if !validate_instance(object.clone()) {
    return Err(errors::ServerError::BadRequest.into());
  }

  let instance_id = database
    .send(CreateNewInstance::new(object.clone()))
    .await??;
  let instance = fetch_instance(database.clone(), instance_id).await?;
  let executor = database
    .send(FetchExecutorByTag::new(object.executor.to_owned()))
    .await??;

  Ok((instance_id, instance, executor.unwrap()))
}

fn validate_instance(object: InstanceRequest) -> bool {
  if object.executor.trim() == "" {
    return false; // invalid executor
  }

  if object.files.len() == 0 {
    return false; // are you sure????
  }

  if object.files.iter().any(|w| !is_valid_pathname(&w.title)) {
    return false; // invalid pathname
  }

  true
}

fn is_valid_pathname(path: &str) -> bool {
  // does not support relative (dot) path
  if path
    .split("/")
    .map(|w| w.trim())
    .any(|w| w == "." || w == ".." || w == "")
  {
    return false;
  }

  let path = Path::new(path);
  if path.is_dir() {
    return false;
  }

  if path.is_absolute() {
    return false;
  }

  return true;
}

pub async fn fetch_instance(
  database: Addr<DbExecutor>,
  instance_id: i64,
) -> Result<InstanceResponse, Error> {
  let instance_with_executor = database.send(FetchInstance::new(instance_id)).await??;
  if instance_with_executor.is_none() {
    return Err(errors::ServerError::ResourceNotFound.into());
  }

  let dependencies = database
    .send(FetchDependenciesByInstanceId::new(instance_id))
    .await??;
  let files = database
    .send(FetchFilesByInstanceId::new(instance_id))
    .await??;

  Ok(InstanceResponse::new(
    instance_with_executor.unwrap(),
    dependencies,
    files,
  ))
}
