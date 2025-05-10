use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct OrganizationLimits {
    pub max_users: u32,
    pub max_attendance_logs: u32,
}
