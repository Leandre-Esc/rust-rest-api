use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use crate::application::user::UserService;
use crate::domain::error::AppError;
use crate::web::dto::{CreateUserRequest, UserResponse, UsersResponse};

pub async fn create_user_handler(
    State(service): State<Arc<UserService>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let command = payload.into();

    let user = service.create_user(command).await?;

    let response = UserResponse::from(user);
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_all_user_handler(
    State(service): State<Arc<UserService>>,
) -> Result<impl IntoResponse, AppError> {
    let users = service.get_all_user().await?;

    let response = UsersResponse::from(users);
    Ok((StatusCode::OK, Json(response)))
}