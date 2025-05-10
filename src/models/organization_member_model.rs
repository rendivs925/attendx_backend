use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::models::user::role::Role;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrganizationMember {
    pub organization_id: ObjectId,

    pub name: String,

    pub role: Role,

    #[serde(default)]
    pub identifiers: HashMap<String, String>,

    #[serde(default = "Utc::now")]
    pub joined_at: DateTime<Utc>,
}
