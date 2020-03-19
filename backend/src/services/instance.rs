use diesel::prelude::*;
use serde::Serialize;
use crate::models::{
  Dependency, Executor, File, Instance, InstanceObject, InstanceStatus, NewDependency, NewFile,
  NewInstance,
};
#[derive(Clone, Debug, Serialize)]
pub struct InstanceDependencyResponse {
  name_with_version: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct InstanceFileResponse {
  title: String,
  content: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct InstanceResponse {
  title: String,
  executor: String,
  status: InstanceStatus,
  result: Option<String>,
  dependencies: Vec<InstanceDependencyResponse>,
  files: Vec<InstanceFileResponse>,
}

pub fn fetch_instance(
  conn: &MysqlConnection,
  instance_id: u64,
) -> Result<Option<InstanceResponse>, diesel::result::Error> {
  use crate::schema::executors::dsl::*;
  use crate::schema::instances::dsl::*;

  let tuple = instances
    .find(instance_id as i64)
    .inner_join(executors)
    .get_result::<(Instance, Executor)>(conn)
    .optional()?;

  let (instance, executor) = match tuple {
    Some(values) => values,
    None => return Ok(None),
  };

  let dependencies = Dependency::belonging_to(&instance).load::<Dependency>(conn)?;
  let dependencies = dependencies
    .iter()
    .map(|w| InstanceDependencyResponse {
      name_with_version: w.name_with_version.to_owned(),
    })
    .collect();

  let files = File::belonging_to(&instance).load::<File>(conn)?;
  let files = files
    .iter()
    .map(|w| InstanceFileResponse {
      title: w.title.to_owned(),
      content: w.content.to_owned(),
    })
    .collect();

  Ok(Some(InstanceResponse {
    title: instance.title,
    executor: executor.name,
    status: instance.status,
    result: instance.result,
    dependencies,
    files,
  }))
}
