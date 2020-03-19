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
