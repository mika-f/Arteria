use diesel::*;
use diesel_derive_enum::*;
use serde::{Deserialize, Serialize};

use crate::models::{DependencySlim, Executor, FileSlim};
use crate::schema::instances;

#[derive(Clone, Debug, DbEnum, Serialize, Deserialize)]
pub enum InstanceStatus {
  Running,
  Success,
  Failure,
  Terminate,
}
#[derive(Clone, Debug, Associations, Identifiable, Queryable)]
#[belongs_to(Executor)]
pub struct Instance {
  pub id: i64,
  pub title: String,
  pub executor_id: i32,
  pub status: InstanceStatus,
  pub result: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstanceRequest {
  pub title: String,
  pub executor: String,
  pub dependencies: Vec<DependencySlim>,
  pub files: Vec<FileSlim>,
}

#[derive(Clone, Debug, Serialize)]
pub struct InstanceResponse {
  pub title: String,
  pub executor: String,
  pub status: InstanceStatus,
  pub result: Option<String>,
  pub dependencies: Vec<DependencySlim>,
  pub files: Vec<FileSlim>,
}

impl InstanceResponse {
  pub fn new(
    instance: InstanceWithExecutor,
    dependencies: Vec<DependencySlim>,
    files: Vec<FileSlim>,
  ) -> Self {
    InstanceResponse {
      title: instance.title,
      executor: instance.executor,
      status: instance.status,
      result: instance.result,
      dependencies,
      files,
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstanceWithExecutor {
  pub title: String,
  pub executor: String,
  pub status: InstanceStatus,
  pub result: Option<String>,
}

#[derive(Clone, Debug, Insertable)]
#[table_name = "instances"]
pub struct NewInstance {
  pub title: String,
  pub executor_id: i32,
  pub status: InstanceStatus,
  pub result: Option<String>,
}

/**
 * Arteria instance
 */

#[derive(Clone, Debug, AsChangeset)]
#[table_name = "instances"]
pub struct InstanceChangeset {
  pub title: Option<String>,
  pub status: Option<InstanceStatus>,
  pub result: Option<String>,
}
