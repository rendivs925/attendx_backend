use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum AttendanceStatus {
    Present,
    Absent,
    Late,
    Excused,
}
