use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::user::{CreateUserCommand, User};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

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

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            full_name: format!("{} {}", user.first_name.unwrap_or_default(), user.last_name.unwrap_or_default()),
            username: user.username,
            email: user.email,
            created_at: user.created_at
        }
    }
}

#[derive(Serialize)]
pub struct UsersResponse(pub Vec<UserResponse>);

impl From<Vec<User>> for UsersResponse {
    fn from(users: Vec<User>) -> Self {
        Self(users.into_iter().map(UserResponse::from).collect())
    }
}