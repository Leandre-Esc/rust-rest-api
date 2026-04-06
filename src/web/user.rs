use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use crate::application::user::UserService;
use crate::domain::error::AppError;
use crate::web::dto::{CreateUserRequest, GetUserByEmailRequest, UserResponse, UsersResponse};

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

pub async fn get_user_by_email_handler(
    State(service): State<Arc<UserService>>,
    Json(payload): Json<GetUserByEmailRequest>
) -> Result<impl IntoResponse, AppError> {
    let user = service.get_user_by_email(&payload.email).await?;

    match user {
        Some(user) => {
            let response = UserResponse::from(user);
            Ok((StatusCode::OK, Json(response)).into_response())
        },
        None => Err(AppError::NotFound(format!("User with email {} not found", payload.email))),
    }
}