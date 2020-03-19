use diesel::prelude::*;
use serde::Serialize;

use crate::models::{
  Dependency, Executor, File, Instance, InstanceObject, InstanceStatus, NewDependency, NewFile,
  NewInstance,
};

pub trait DependencyPayload {
  fn name_with_version(&self) -> &str;
}

pub trait FilePayload {
  fn title(&self) -> &str;
  fn content(&self) -> &str;
}

pub trait InstancePayload<D: DependencyPayload, F: FilePayload> {
  fn title(&self) -> &str;
  fn version_str(&self) -> &str;
  fn dependencies(&self) -> Vec<&D>;
  fn files(&self) -> Vec<&F>;
}

pub fn create_instance<T: InstancePayload<D, F>, D: DependencyPayload, F: FilePayload>(
  connection: &MysqlConnection,
  data: T,
) -> Result<Option<i64>, diesel::result::Error> {
  let item = connection.transaction::<_, diesel::result::Error, _>(|| {
    let executor = match Executor::find_by_tag(connection, &data.version_str())? {
      Some(version) => version,
      None => return Ok(None),
    };

    let instance = Instance::insert(connection, NewInstance::new(&data.title(), executor.id))?;

    let deps = Dependency::insert(
      connection,
      data
        .dependencies()
        .iter()
        .map(|w| NewDependency::new(instance.id, w.name_with_version()))
        .collect(),
    )?;

    let files = File::insert(
      connection,
      data
        .files()
        .iter()
        .map(|w| NewFile::new(instance.id, w.title(), w.content()))
        .collect(),
    )?;

    Ok(Some(instance.id))
  })?;

  Ok(item)
}

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
