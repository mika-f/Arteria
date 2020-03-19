use diesel::prelude::*;
use diesel::*;

use crate::schema::executors;

/**
 * Perl Executors that runnable in Arteria
 */
#[derive(Debug, Clone, Identifiable, Queryable)]
pub struct Executor {
  pub id: i32,
  pub name: String,
  pub image: String,
  pub tag: String,
}

impl Executor {
  pub fn find_by_tag(
    conn: &MysqlConnection,
    tag: &str,
  ) -> Result<Option<Executor>, diesel::result::Error> {
    use crate::schema::executors::dsl::executors;

    let item = executors
      .filter(crate::schema::executors::dsl::tag.eq(tag))
      .first(conn)
      .optional()?;

    Ok(item)
  }
}
