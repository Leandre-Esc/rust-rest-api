use crate::domain::user::{CreateUserCommand, User, UserRepository};
use std::sync::Arc;
use bcrypt::{hash, DEFAULT_COST};
use crate::domain::error::AppError;

pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, mut cmd: CreateUserCommand) -> Result<User, AppError> {
        // Check if user already exist
        if self.repo.is_exists(&cmd.email).await {
            return Err(AppError::AlreadyExists("User already exists".to_string()));
        }

        // Hashed password with bcrypt
        let hashed_password = hash(cmd.password, DEFAULT_COST)
            .map_err(|e| AppError::Internal(e.to_string()))?;
        cmd.password = hashed_password;

        self.repo.save(cmd).await
            .map_err(|e| AppError::Internal(e))
    }

    pub async fn get_all_user(&self) -> Result<Vec<User>, AppError> {
        self.repo.get_all().await.map_err(|e| AppError::Internal(e.to_string()))
    }
}