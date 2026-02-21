use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use crate::application::user::UserService;
use crate::domain::error::AppError;
use crate::web::dto::{CreateUserRequest, UserResponse};

pub async fn create_user_handler(
    State(service): State<Arc<UserService>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let command = payload.into();

    let user = service.create_user(command).await?;

    let response = UserResponse::from(user);
    Ok((StatusCode::CREATED, Json(response)))
}