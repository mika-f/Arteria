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
