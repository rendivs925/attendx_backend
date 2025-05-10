use crate::{
    models::user_model::User,
    repositories::user_repository::UserRepository,
    types::{
        models::user::defaults::default_status,
        requests::{
            auth::register_request::RegisterRequest, user::update_user_request::UpdateUserRequest,
        },
    },
    utils::auth_utils::{generate_jwt, hash_password, verify_password},
};
use anyhow::{Context, Result, anyhow};
use bson::oid::ObjectId;
use chrono::Utc;
use serde_json::Value;
use std::{collections::HashSet, sync::Arc};

pub struct UserService {
    pub user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn authenticate_user(
        &self,
        email: &str,
        password: &str,
        messages: &Value,
    ) -> Result<(User, String)> {
        let user = self
            .user_repository
            .find_user("email", email)
            .await
            .context(format!("User with email '{}' not found", email))?
            .ok_or_else(|| {
                anyhow!(
                    messages
                        .get("fetch.not_found")
                        .and_then(Value::as_str)
                        .map(String::from)
                        .unwrap_or_else(|| "User not found".to_string())
                )
            })?;

        if !verify_password(password, &user.password).map_err(|err| {
            anyhow!(
                "Password verification failed for user '{}': {:?}",
                user.email,
                err
            )
        })? {
            return Err(anyhow!(
                messages
                    .get("auth.invalid_credentials")
                    .and_then(Value::as_str)
                    .map(String::from)
                    .unwrap_or_else(|| "Invalid credentials".to_string())
            ));
        }

        let token = generate_jwt(&user.name, email)
            .map_err(|e| anyhow!("JWT generation failed for user '{}': {}", user.email, e))
            .context("Failed to generate JWT")?;

        Ok((user, token))
    }

    pub async fn create_user(&self, new_user: RegisterRequest, messages: &Value) -> Result<User> {
        if let Some(_existing) = self
            .user_repository
            .find_user("email", new_user.email.as_str())
            .await
            .context(format!(
                "Error checking existing user with email '{}'",
                new_user.email
            ))?
        {
            return Err(anyhow!(
                messages
                    .get("create.duplicate")
                    .and_then(Value::as_str)
                    .map(String::from)
                    .unwrap_or_else(|| "Duplicate email or phone number".to_string())
            )
            .context("Duplicate email or phone number"));
        }

        let hashed_password = hash_password(&new_user.password).map_err(|e| {
            anyhow!(
                "Failed to hash password for user '{}': {}",
                new_user.email,
                e
            )
        })?;

        let now = Utc::now();

        let user = User {
            _id: Some(ObjectId::new()),
            name: new_user.name,
            email: new_user.email.clone(),
            password: hashed_password,
            organization_ids: HashSet::new(),
            owned_organizations: 0,
            subscription_plan: new_user.subscription_plan,
            status: default_status(),
            created_at: now,
            updated_at: now,
        };

        self.user_repository.create_user(&user).await.map_err(|e| {
            anyhow!(
                messages
                    .get("create.success")
                    .and_then(Value::as_str)
                    .map(String::from)
                    .unwrap_or_else(|| "DB insert failed".to_string())
            )
            .context(format!("DB insert failed: {}", e))
        })
    }

    pub async fn get_all_users(&self, messages: &Value) -> Result<Vec<User>> {
        self.user_repository.get_all_users().await.map_err(|e| {
            anyhow!(
                messages
                    .get("fetch.all_success")
                    .and_then(Value::as_str)
                    .map(String::from)
                    .unwrap_or_else(|| "Error fetching users".to_string())
            )
            .context(format!("Error fetching users: {}", e))
        })
    }

    pub async fn get_user(&self, email: &str, messages: &Value) -> Result<Option<User>> {
        self.user_repository
            .find_user("email", email)
            .await
            .map_err(|e| {
                anyhow!(
                    messages
                        .get("fetch.success")
                        .and_then(Value::as_str)
                        .map(String::from)
                        .unwrap_or_else(|| "Error retrieving user".to_string())
                )
                .context(format!("Error retrieving user: {}", e))
            })
    }

    pub async fn update_user(
        &self,
        email: &str,
        user: UpdateUserRequest,
        messages: &Value,
    ) -> Result<UpdateUserRequest> {
        self.user_repository
            .update_user(email, user)
            .await
            .map_err(|e| {
                anyhow!(
                    messages
                        .get("update.success")
                        .and_then(Value::as_str)
                        .map(String::from)
                        .unwrap_or_else(|| "Error updating user".to_string())
                )
                .context(format!("Error updating user: {}", e))
            })
    }

    pub async fn delete_user(&self, email: &str, messages: &Value) -> Result<()> {
        self.user_repository.delete_user(email).await.map_err(|e| {
            anyhow!(
                messages
                    .get("delete.success")
                    .and_then(Value::as_str)
                    .map(String::from)
                    .unwrap_or_else(|| "Error deleting user".to_string())
            )
            .context(format!("Error deleting user: {}", e))
        })
    }
}
