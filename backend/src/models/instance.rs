use diesel::prelude::*;
use diesel::*;
use diesel_derive_enum::*;
use serde::Serialize;

use crate::models::Version;
use crate::schema::instances;

#[derive(Clone, Debug, DbEnum, Serialize)]
pub enum InstanceStatus {
  Running,
  Success,
  Failure,
  Terminate,
}

/**
 * Arteria instance
 */
#[derive(Clone, Debug, Associations, Identifiable, Queryable)]
#[belongs_to(Version)]
pub struct Instance {
  pub id: i64,
  pub title: String,
  pub version_id: i32,
  pub status: InstanceStatus,
  pub result: Option<String>,
}

#[derive(Clone, Debug, Insertable)]
#[table_name = "instances"]
pub struct NewInstance<'a> {
  pub title: &'a str,
  pub version_id: i32,
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

impl Instance {
  pub fn find(
    conn: &MysqlConnection,
    instance_id: i64,
  ) -> Result<Option<Instance>, diesel::result::Error> {
    use crate::schema::instances::dsl::*;

    let item = instances
      .find(instance_id)
      .get_result::<Instance>(conn)
      .optional()?;
    Ok(item)
  }

  pub fn find_with_version(
    conn: &MysqlConnection,
    instance_id: i64,
  ) -> Result<Option<(Instance, Version)>, diesel::result::Error> {
    use crate::schema::instances::dsl::*;
    use crate::schema::versions::dsl::*;

    let item = instances
      .find(instance_id)
      .inner_join(versions)
      .get_result::<(Instance, Version)>(conn)
      .optional()?;
    Ok(item)
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
