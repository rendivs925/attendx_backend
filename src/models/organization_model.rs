use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::models::organization::organization_limit::OrganizationLimits;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Organization {
    #[serde(default)]
    pub _id: Option<ObjectId>,

    pub name: String,

    pub email: String,

    pub owner_id: ObjectId,

    pub password: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,

    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,

    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,

    pub limits: OrganizationLimits,
}
