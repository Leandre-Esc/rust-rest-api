use std::sync::Arc;

use bcrypt::{DEFAULT_COST, hash};
use uuid::Uuid;
use crate::shared::error::AppError;
use crate::users::domain::{CreateUserCommand, UpdateUserCommand, User, UserRepository};
use crate::users::dto::UpdateUserRequest;

pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_all_user(&self) -> Result<Vec<User>, AppError> {
        self.repository.get_all().await.map_err(AppError::Internal)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        self.repository
            .get_by_email(email)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn create_user(&self, mut cmd: CreateUserCommand) -> Result<User, AppError> {
        if self.repository.is_exists(&cmd.email).await {
            return Err(AppError::AlreadyExists("User already exists".to_string()));
        }

        let hashed_password =
            hash(cmd.password, DEFAULT_COST).map_err(|e| AppError::Internal(e.to_string()))?;
        cmd.password = hashed_password;

        self.repository
            .create(cmd)
            .await
            .map_err(AppError::Internal)
    }

    pub async fn update_user(&self, id: Uuid, req: UpdateUserRequest) -> Result<User, AppError> {
        let existing = self
            .repository
            .get_by_id(id)
            .await
            .map_err(AppError::Internal)?
            .ok_or_else(|| AppError::NotFound(format!("User {} not found", id)))?;

        let password = match req.password {
            Some(plain) => Some (
                hash(plain, DEFAULT_COST).map_err(|e| AppError::Internal(e.to_string()))?
            ),
            None => None,
        };

        let cmd = UpdateUserCommand {
            id,
            first_name: req.first_name.or(existing.first_name),
            last_name: req.last_name.or(existing.last_name),
            username: req.username.unwrap_or(existing.username).into(),
            email: req.email.unwrap_or(existing.email).into(),
            password: password.unwrap_or(existing.password).into(),
        };

        self.repository.update(cmd).await.map_err(AppError::Internal)
    }
}
