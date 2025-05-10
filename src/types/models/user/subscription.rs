use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::models::organization::organization_limit::OrganizationLimits;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Subscription {
    pub plan: SubscriptionPlan,
    pub status: SubscriptionStatus,
    pub start_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
}

impl Subscription {
    pub fn is_active(&self) -> bool {
        self.status == SubscriptionStatus::Active && Utc::now() < self.expiry_date
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum SubscriptionPlan {
    Free,
    Pro,
    Enterprise,
}

impl SubscriptionPlan {
    pub fn max_organizations(&self) -> u32 {
        match self {
            SubscriptionPlan::Free => 1,
            SubscriptionPlan::Pro => 5,
            SubscriptionPlan::Enterprise => 50,
        }
    }

    pub fn default_limits(&self) -> OrganizationLimits {
        match self {
            SubscriptionPlan::Free => OrganizationLimits {
                max_users: 10,
                max_attendance_logs: 5_000,
            },
            SubscriptionPlan::Pro => OrganizationLimits {
                max_users: 500,
                max_attendance_logs: 500_000,
            },
            SubscriptionPlan::Enterprise => OrganizationLimits {
                max_users: 10_000,
                max_attendance_logs: 50_000_000,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum SubscriptionStatus {
    Active,
    Expired,
    Canceled,
}
