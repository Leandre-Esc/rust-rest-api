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
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

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
    async fn create(&self, cmd: CreateUserCommand) -> Result<User, String>;
    async fn get_all(&self) -> Result<Vec<User>, String>;
    async fn get_by_email(&self, email: &str) -> Result<Option<User>, String>;
}