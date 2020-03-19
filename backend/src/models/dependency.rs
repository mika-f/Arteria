use diesel::prelude::*;
use diesel::*;

use crate::models::Instance;
use crate::schema::dependencies;

/**
 * Module dependency(ies) of specified instance
 */
#[derive(Clone, Debug, Associations, Identifiable, Queryable)]
#[table_name = "dependencies"]
#[belongs_to(Instance)]
pub struct Dependency {
  pub id: i64,
  pub instance_id: i64,
  pub name_with_version: String,
}

#[derive(Clone, Debug, Insertable)]
#[table_name = "dependencies"]
pub struct NewDependency<'a> {
  pub instance_id: i64,
  pub name_with_version: &'a str,
}

impl Dependency {
  pub fn insert(
    conn: &MysqlConnection,
    items: Vec<NewDependency>,
  ) -> Result<Vec<Dependency>, diesel::result::Error> {
    use crate::schema::dependencies::dsl::*;

    let items = conn.transaction::<_, diesel::result::Error, _>(|| {
      let inserted_count = diesel::insert_into(dependencies)
        .values(items)
        .execute(conn)?;

      Ok(
        dependencies
          .order(id.desc())
          .limit(inserted_count as i64)
          .get_results(conn)?,
      )
    })?;

    Ok(items)
  }
}