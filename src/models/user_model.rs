use crate::types::models::user::{
    defaults::{default_status, default_subscription_plan},
    subscription::SubscriptionPlan,
    user_status::UserStatus,
};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(default)]
    pub _id: Option<ObjectId>,
    pub name: String,

    pub email: String,

    pub password: String,

    #[serde(default)]
    pub organization_ids: HashSet<ObjectId>,

    #[serde(default)]
    pub owned_organizations: u32,

    #[serde(default = "default_subscription_plan")]
    pub subscription_plan: SubscriptionPlan,

    #[serde(default = "default_status")]
    pub status: UserStatus,

    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,

    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}
