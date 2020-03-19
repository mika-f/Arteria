use diesel::prelude::*;
use diesel::*;

use crate::models::Instance;
use crate::schema::files;

#[derive(Clone, Debug, Insertable)]
#[table_name = "files"]
pub struct NewFile<'a> {
  pub instance_id: i64,
  pub title: &'a str,
  pub content: &'a str,
}

impl<'a> NewFile<'a> {
  pub fn new(instance_id: i64, title: &'a str, content: &'a str) -> Self {
    NewFile {
      instance_id,
      title,
      content,
    }
  }
}

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

impl File {
  pub fn insert(
    conn: &MysqlConnection,
    items: Vec<NewFile>,
  ) -> Result<Vec<File>, diesel::result::Error> {
    use crate::schema::files::dsl::*;

    let items = conn.transaction::<_, diesel::result::Error, _>(|| {
      let inserted_count = diesel::insert_into(files).values(items).execute(conn)?;

      Ok(
        files
          .order(id.desc())
          .limit(inserted_count as i64)
          .get_results(conn)?,
      )
    })?;

    Ok(items)
  }
}
