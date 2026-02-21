use crate::domain::user::{CreateUserCommand, User, UserRepository};
use std::sync::Arc;

pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, cmd: CreateUserCommand) -> Result<User, String> {
        // C'est ici qu'on hacherait le mot de passe avant de l'envoyer au repo
        self.repo.save(cmd).await
    }
}