use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Command {
  pub command: String,
  pub value: Option<String>,
}
