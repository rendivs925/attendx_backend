use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
}
