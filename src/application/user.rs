use crate::domain::user::{CreateUserCommand, User, UserRepository};
use std::sync::Arc;
use bcrypt::{hash, DEFAULT_COST};

pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, mut cmd: CreateUserCommand) -> Result<User, String> {
        // Check if user already exist
        if self.repo.is_exists(&cmd.email).await {
            return Err("User already exists".to_string());
        }

        // Hashed password with bcrypt
        let hashed_password = hash(cmd.password, DEFAULT_COST).unwrap();
        cmd.password = hashed_password;

        self.repo.save(cmd).await
    }
}