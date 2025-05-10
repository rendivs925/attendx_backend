use crate::types::models::user::{
    role::Role, subscription::SubscriptionPlan, user_status::UserStatus,
};

pub fn default_role() -> Role {
    Role::User
}

pub fn default_subscription_plan() -> SubscriptionPlan {
    SubscriptionPlan::Free
}

pub fn default_status() -> UserStatus {
    UserStatus::Active
}
