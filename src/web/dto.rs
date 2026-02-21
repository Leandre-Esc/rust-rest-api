use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::user::CreateUserCommand;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

// Conversion facile du DTO vers la commande de domaine
impl From<CreateUserRequest> for CreateUserCommand {
    fn from(req: CreateUserRequest) -> Self {
        Self {
            first_name: req.first_name,
            last_name: req.last_name,
            username: req.username,
            email: req.email,
            password: req.password,
        }
    }
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub full_name: String,
    pub username: String,
    pub email: String,
    pub created_at: Option<DateTime<Utc>>,
}