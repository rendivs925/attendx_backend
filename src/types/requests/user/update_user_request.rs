use crate::types::models::user::subscription::SubscriptionPlan;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_plan: Option<SubscriptionPlan>,
}
