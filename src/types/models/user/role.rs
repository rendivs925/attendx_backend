use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum Role {
    Admin,
    Staff,
    User,
}
