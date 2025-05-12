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
use anyhow::anyhow;
use anyhow::{Context, Result};
use bson::oid::ObjectId;
use chrono::Utc;
use std::{collections::HashSet, sync::Arc};

#[derive(Debug)]
pub enum UserServiceError {
    NotFound,
    InvalidCredentials,
    DuplicateEmail,
    DbError(String),
    JwtGenerationError(String),
    PasswordHashingError(String),
}

impl UserServiceError {
    fn to_message(&self, messages: &Messages) -> String {
        match self {
            UserServiceError::NotFound => {
                messages.get_user_message("fetch.not_found", "User not found")
            }
            UserServiceError::InvalidCredentials => {
                messages.get_auth_message("login.invalid_credentials", "Invalid credentials")
            }
            UserServiceError::DuplicateEmail => {
                messages.get_auth_message("register.duplicate", "Duplicate email")
            }
            UserServiceError::DbError(_) => messages.get_auth_message(
                "register.db_error",
                "Database error occurred during user registration",
            ),
            UserServiceError::JwtGenerationError(_) => {
                messages.get_auth_message("auth.jwt_generation_failed", "JWT generation failed")
            }
            UserServiceError::PasswordHashingError(_) => {
                messages.get_auth_message("auth.password_hashing_failed", "Password hashing failed")
            }
        }
    }
}

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
            .context(UserServiceError::NotFound.to_message(messages))?
            .ok_or_else(|| anyhow!(UserServiceError::NotFound.to_message(messages)))?;

        verify_password(password, &user.password)
            .map_err(|_| anyhow!(UserServiceError::InvalidCredentials.to_message(messages)))?;

        let token = generate_jwt(&user.name, &user.email).map_err(|e| {
            anyhow!(UserServiceError::JwtGenerationError(e.to_string()).to_message(messages))
        })?;

        Ok((user, token))
    }

    pub async fn register_user(
        &self,
        new_user: RegisterRequest,
        messages: &Messages,
    ) -> Result<User> {
        let existing_user = self
            .user_repository
            .find_user("email", &new_user.email)
            .await
            .context(
                UserServiceError::DbError("Failed to check for duplicate email".into())
                    .to_message(messages),
            )?;

        if existing_user.is_some() {
            return Err(anyhow!(
                UserServiceError::DuplicateEmail.to_message(messages)
            ));
        }

        let hashed_password = hash_password(&new_user.password).map_err(|e| {
            anyhow!(UserServiceError::PasswordHashingError(e.to_string()).to_message(messages))
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

        self.user_repository
            .register_user(&user)
            .await
            .map_err(|e| anyhow!(UserServiceError::DbError(e.to_string()).to_message(messages)))?;

        Ok(user)
    }

    pub async fn get_all_users(&self, messages: &Messages) -> Result<Vec<User>> {
        self.user_repository
            .get_all_users()
            .await
            .map_err(|e| anyhow!(UserServiceError::DbError(e.to_string()).to_message(messages)))
    }

    pub async fn get_user(&self, email: &str, messages: &Messages) -> Result<Option<User>> {
        self.user_repository
            .find_user("email", email)
            .await
            .map_err(|e| anyhow!(UserServiceError::DbError(e.to_string()).to_message(messages)))
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
            .map_err(|e| anyhow!(UserServiceError::DbError(e.to_string()).to_message(messages)))
    }

    pub async fn delete_user(&self, email: &str, messages: &Messages) -> Result<()> {
        self.user_repository
            .delete_user(email)
            .await
            .map_err(|e| anyhow!(UserServiceError::DbError(e.to_string()).to_message(messages)))
    }
}
