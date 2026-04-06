use std::sync::Arc;

use bcrypt::{DEFAULT_COST, hash};

use crate::shared::error::AppError;
use crate::users::domain::{CreateUserCommand, User, UserRepository};

pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
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

    pub async fn get_all_user(&self) -> Result<Vec<User>, AppError> {
        self.repository.get_all().await.map_err(AppError::Internal)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        self.repository
            .get_by_email(email)
            .await
            .map_err(AppError::Internal)
    }
}
