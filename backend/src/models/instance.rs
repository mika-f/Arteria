use diesel::prelude::*;
use diesel::*;
use diesel_derive_enum::*;
use serde::Serialize;

use crate::models::{Dependency, Executor, File};
use crate::schema::instances;

#[derive(Clone, Debug, DbEnum, Serialize)]
pub enum InstanceStatus {
  Running,
  Success,
  Failure,
  Terminate,
}

#[derive(Clone, Debug, Insertable)]
#[table_name = "instances"]
pub struct NewInstance<'a> {
  pub title: &'a str,
  pub executor_id: i32,
  pub status: InstanceStatus,
  pub result: Option<String>,
}

impl<'a> NewInstance<'a> {
  pub fn new(title: &'a str, executor_id: i32) -> Self {
    NewInstance {
      title,
      executor_id,
      status: InstanceStatus::Running,
      result: None,
    }
  }
}

/**
 * Arteria instance
 */
#[derive(Clone, Debug, Associations, Identifiable, Queryable)]
#[belongs_to(Executor)]
pub struct Instance {
  pub id: i64,
  pub title: String,
  pub executor_id: i32,
  pub status: InstanceStatus,
  pub result: Option<String>,
}

#[derive(Clone, Debug, AsChangeset)]
#[table_name = "instances"]
pub struct InstanceChangeset {
  pub title: Option<String>,
  pub status: Option<InstanceStatus>,
  pub result: Option<String>,
}

#[derive(Clone, Debug)]
pub struct InstanceObject {
  pub id: i64,
  pub title: String,
  pub status: InstanceStatus,
  pub result: Option<String>,

  // relationships
  pub executor: Executor,
  pub dependencies: Vec<Dependency>,
  pub files: Vec<File>,
}

impl Instance {
  pub fn find(
    conn: &MysqlConnection,
    instance_id: i64,
  ) -> Result<Option<InstanceObject>, diesel::result::Error> {
    use crate::schema::executors::dsl::*;
    use crate::schema::instances::dsl::*;

    let item = instances
      .find(instance_id)
      .inner_join(executors)
      .get_result::<(Instance, Executor)>(conn)
      .optional()?;

    let (instance, executor) = match item {
      Some(values) => values,
      None => return Ok(None),
    };

    let dependencies = Dependency::belonging_to(&instance).load::<Dependency>(conn)?;
    let files = File::belonging_to(&instance).load::<File>(conn)?;

    Ok(Some(InstanceObject {
      id: instance.id,
      title: instance.title,
      status: instance.status,
      result: instance.result,
      executor,
      dependencies,
      files,
    }))
  }

  pub fn insert(
    conn: &MysqlConnection,
    instance: NewInstance,
  ) -> Result<Instance, diesel::result::Error> {
    use crate::schema::instances::dsl::*;

    let item = conn.transaction::<_, diesel::result::Error, _>(|| {
      let inserted_count = diesel::insert_into(instances)
        .values(instance)
        .execute(conn)?;

      Ok(
        instances
          .order(id.desc())
          .limit(inserted_count as i64) // always 1
          .get_result(conn)?,
      )
    })?;

    Ok(item)
  }

  pub fn update(
    conn: &MysqlConnection,
    instance_id: i64,
    instance: InstanceChangeset,
  ) -> Result<Instance, diesel::result::Error> {
    use crate::schema::instances::dsl::*;

    let item = conn.transaction::<_, diesel::result::Error, _>(|| {
      diesel::update(instances::find(instances, instance_id)) // Why required 1st argument?
        .set(instance)
        .execute(conn)?;

      Ok(instances.find(instance_id).get_result(conn)?)
    })?;

    Ok(item)
  }
}
