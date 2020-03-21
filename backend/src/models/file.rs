use diesel::*;
use serde::{Deserialize, Serialize};

use crate::models::Instance;
use crate::schema::files;

#[derive(Clone, Debug, Associations, Identifiable, Queryable, Deserialize, Serialize)]
#[belongs_to(Instance)]
pub struct File {
  pub id: i64,
  pub instance_id: i64,
  pub title: String,
  pub content: String,
}

#[derive(Clone, Debug, Queryable, Deserialize, Serialize)]
pub struct FileSlim {
  pub title: String,
  pub content: String,
}

#[derive(Clone, Debug, Insertable, Deserialize, Serialize)]
#[table_name = "files"]
pub struct NewFile {
  pub instance_id: i64,
  pub title: String,
  pub content: String,
}
