use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
}

// Objet de transfert interne au domaine pour la création
pub struct CreateUserCommand {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn is_exists(&self, email: &str) -> bool;
    async fn save(&self, cmd: CreateUserCommand) -> Result<User, String>;
}