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
        locale_utils::Messages,
    },
};
use anyhow::{Result, anyhow};
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
            .map_err(|_| {
                anyhow!(
                    "{}",
                    messages.get_user_message("fetch.not_found", "User not found"),
                )
            })?
            .ok_or_else(|| {
                anyhow!(messages.get_user_message("fetch.not_found", "User not found"))
            })?;

        if !verify_password(password, &user.password)
            .map_err(|err| anyhow!("Password verification failed: {:?}", err))?
        {
            return Err(anyhow!(messages.get_auth_message(
                "login.invalid_credentials",
                "Invalid credentials"
            )));
        }

        let token =
            generate_jwt(&user.name, email).map_err(|e| anyhow!("JWT generation failed: {}", e))?;

        Ok((user, token))
    }

    pub async fn register_user(
        &self,
        new_user: RegisterRequest,
        messages: &Messages,
    ) -> Result<User> {
        if let Some(_) = self
            .user_repository
            .find_user("email", &new_user.email)
            .await
            .map_err(|_| {
                anyhow!(messages.get_auth_message("register.duplicate", "Duplicate email"))
            })?
        {
            return Err(anyhow!(
                messages.get_auth_message("register.duplicate", "Duplicate email")
            ));
        }

        let hashed_password = hash_password(&new_user.password)
            .map_err(|e| anyhow!("Failed to hash password for '{}': {}", new_user.email, e))?;

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

        self.user_repository
            .register_user(&user)
            .await
            .map_err(|e| {
                anyhow!(
                    "{}: {}",
                    messages.get_auth_message("register.success", "DB insert failed"),
                    e
                )
            })
    }

    pub async fn get_all_users(&self, messages: &Messages) -> Result<Vec<User>> {
        self.user_repository.get_all_users().await.map_err(|e| {
            anyhow!(
                "{}: {}",
                messages.get_user_message("fetch.all_success", "Error fetching users"),
                e
            )
        })
    }

    pub async fn get_user(&self, email: &str, messages: &Messages) -> Result<Option<User>> {
        self.user_repository
            .find_user("email", email)
            .await
            .map_err(|e| {
                anyhow!(
                    "{}: {}",
                    messages.get_user_message("fetch.success", "Error retrieving user"),
                    e
                )
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
                anyhow!(
                    "{}: {}",
                    messages.get_user_message("update.success", "Error updating user"),
                    e
                )
            })
    }

    pub async fn delete_user(&self, email: &str, messages: &Messages) -> Result<()> {
        self.user_repository.delete_user(email).await.map_err(|e| {
            anyhow!(
                "{}: {}",
                messages.get_user_message("delete.success", "Error deleting user"),
                e
            )
        })
    }
}
