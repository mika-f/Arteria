use diesel::prelude::*;
use diesel::*;

use crate::models::Instance;
use crate::schema::files;

/**
 * Perl files that related to instance
 */
#[derive(Clone, Debug, Associations, Identifiable, Queryable)]
#[belongs_to(Instance)]
pub struct File {
  pub id: i64,
  pub instance_id: i64,
  pub title: String,
  pub content: String,
}
