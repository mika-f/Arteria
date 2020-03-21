use diesel::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct ExecutorResponse {
  pub name: String,
  pub tag: String,
}
