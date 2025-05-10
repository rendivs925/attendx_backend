use crate::{
    models::user_model::User,
    repositories::user_repository::UserRepository,
    types::{
        models::user::defaults::default_status,
        requests::{
            auth::register_request::RegisterRequest, user::update_user_request::UpdateUserRequest,
        },
    },
    utils::{
        auth_utils::{generate_jwt, hash_password, verify_password},
        locale_utils::{Messages, Namespace},
    },
};
use anyhow::{Context, Result, anyhow};
use bson::oid::ObjectId;
use chrono::Utc;
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
        messages: &Messages,
    ) -> Result<(User, String)> {
        let user = self
            .user_repository
            .find_user("email", email)
            .await
            .context(format!("User with email '{}' not found", email))?
            .ok_or_else(|| {
                anyhow!(messages.get_str(Namespace::User, "fetch.not_found", "User not found",))
            })?;

        if !verify_password(password, &user.password).map_err(|err| {
            anyhow!(
                "Password verification failed for user '{}': {:?}",
                user.email,
                err
            )
        })? {
            return Err(anyhow!(messages.get_str(
                Namespace::User,
                "auth.invalid_credentials",
                "Invalid credentials",
            )));
        }

        let token = generate_jwt(&user.name, email)
            .map_err(|e| anyhow!("JWT generation failed for user '{}': {}", user.email, e))
            .context("Failed to generate JWT")?;

        Ok((user, token))
    }

    pub async fn create_user(
        &self,
        new_user: RegisterRequest,
        messages: &Messages,
    ) -> Result<User> {
        if let Some(_existing) = self
            .user_repository
            .find_user("email", new_user.email.as_str())
            .await
            .context(format!(
                "Error checking existing user with email '{}'",
                new_user.email
            ))?
        {
            return Err(anyhow!(messages.get_str(
                Namespace::User,
                "create.duplicate",
                "Duplicate email or phone number",
            ))
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
            anyhow!(messages.get_str(Namespace::User, "create.success", "DB insert failed",))
                .context(format!("DB insert failed: {}", e))
        })
    }

    pub async fn get_all_users(&self, messages: &Messages) -> Result<Vec<User>> {
        self.user_repository.get_all_users().await.map_err(|e| {
            anyhow!(messages.get_str(Namespace::User, "fetch.all_success", "Error fetching users",))
                .context(format!("Error fetching users: {}", e))
        })
    }

    pub async fn get_user(&self, email: &str, messages: &Messages) -> Result<Option<User>> {
        self.user_repository
            .find_user("email", email)
            .await
            .map_err(|e| {
                anyhow!(
                    messages.get_str(Namespace::User, "fetch.success", "Error retrieving user",)
                )
                .context(format!("Error retrieving user: {}", e))
            })
    }

    pub async fn update_user(
        &self,
        email: &str,
        user: UpdateUserRequest,
        messages: &Messages,
    ) -> Result<UpdateUserRequest> {
        self.user_repository
            .update_user(email, user)
            .await
            .map_err(|e| {
                anyhow!(messages.get_str(Namespace::User, "update.success", "Error updating user",))
                    .context(format!("Error updating user: {}", e))
            })
    }

    pub async fn delete_user(&self, email: &str, messages: &Messages) -> Result<()> {
        self.user_repository.delete_user(email).await.map_err(|e| {
            anyhow!(messages.get_str(Namespace::User, "delete.success", "Error deleting user",))
                .context(format!("Error deleting user: {}", e))
        })
    }
}
