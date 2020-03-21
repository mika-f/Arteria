use diesel::*;
use serde::{Deserialize, Serialize};

use crate::models::Instance;
use crate::schema::dependencies;

#[derive(Clone, Debug, Associations, Identifiable, Queryable, Serialize, Deserialize)]
#[table_name = "dependencies"]
#[belongs_to(Instance)]
pub struct Dependency {
  pub id: i64,
  pub instance_id: i64,
  pub name_with_version: String,
}

#[derive(Clone, Debug, Queryable, Serialize, Deserialize)]
pub struct DependencySlim {
  pub name_with_version: String,
}

#[derive(Clone, Debug, Insertable, Serialize, Deserialize)]
#[table_name = "dependencies"]
pub struct NewDependency {
  pub instance_id: i64,
  pub name_with_version: String,
}
