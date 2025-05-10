use crate::{
    constants::{CLIENT_ID, CLIENT_SECRET, REDIRECT_URI},
    models::user_model::User,
    repositories::user_repository::UserRepository,
    types::models::user::{subscription::SubscriptionPlan, user_status::UserStatus},
};
use actix_web::web;
use bson::oid::ObjectId;
use chrono::Utc;
use reqwest::Client;
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

pub async fn exchange_code_for_token(code: &str) -> Result<HashMap<String, String>, String> {
    let client = Client::new();
    let params = [
        ("code", code),
        ("client_id", &CLIENT_ID),
        ("client_secret", &CLIENT_SECRET),
        ("redirect_uri", &REDIRECT_URI),
        ("grant_type", "authorization_code"),
    ];

    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .map_err(|_| "OAuth2 token exchange request failed".to_string())?;

    let tokens = response
        .json::<HashMap<String, String>>()
        .await
        .map_err(|_| "Failed to parse OAuth2 token response".to_string())?;

    Ok(tokens)
}

pub async fn register_new_user(
    user_repository: web::Data<Arc<UserRepository>>,
    name: String,
    email: String,
) -> Result<User, String> {
    let new_user = User {
        _id: Some(ObjectId::new()),
        name,
        email,
        password: "".to_string(),
        organization_ids: Default::default(),
        subscription_plan: SubscriptionPlan::Free,
        owned_organizations: 0,
        status: UserStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    user_repository
        .create_user(&new_user)
        .await
        .map_err(|e| format!("Database error: {}", e))
}

pub async fn fetch_user_info(access_token: &str) -> Result<Value, String> {
    let client = Client::new();

    let response = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|_| "Failed to send request to fetch user info".to_string())?;

    let user_info = response
        .json::<Value>()
        .await
        .map_err(|_| "Failed to parse user info response".to_string())?;

    Ok(user_info)
}
