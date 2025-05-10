use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum AttendanceType {
    SingleMark,
    DoubleMark,
}
