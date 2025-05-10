use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Display, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Permission {
    MarkAttendance,
    ViewAttendance,
    ManageUsers,
}
