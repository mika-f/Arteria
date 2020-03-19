use diesel::prelude::*;
use diesel::*;

use crate::schema::versions;

/**
 * Perl Versions that runnable in Arteria
 */
#[derive(Debug, Clone, Identifiable, Queryable)]
pub struct Version {
  pub id: i32,
  pub name: String,
  pub tag: String,
}

impl Version {
  pub fn all(conn: &MysqlConnection) -> Result<Vec<Version>, diesel::result::Error> {
    use crate::schema::versions::dsl::*;

    let items = versions.load::<Version>(conn)?;
    Ok(items)
  }

  pub fn find_by_tag(
    conn: &MysqlConnection,
    tag: &str,
  ) -> Result<Option<Version>, diesel::result::Error> {
    use crate::schema::versions::dsl::*;

    let item = versions
      .filter(crate::schema::versions::dsl::tag.eq(tag))
      .get_result(conn)
      .optional()?;

    Ok(item)
  }
}
