use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum::extract::Path;
use uuid::Uuid;
use crate::shared::error::AppError;
use crate::users::{
    dto::{CreateUserRequest, GetUserByEmailRequest, UserResponse, UsersResponse},
    service::UserService,
};
use crate::users::dto::UpdateUserRequest;

pub async fn get_all_user_handler(
    State(service): State<Arc<UserService>>,
) -> Result<impl IntoResponse, AppError> {
    let users = service.get_all_user().await?;

    Ok((StatusCode::OK, Json(UsersResponse::from(users))))
}

pub async fn get_user_by_email_handler(
    State(service): State<Arc<UserService>>,
    Json(payload): Json<GetUserByEmailRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = service.get_user_by_email(&payload.email).await?;

    match user {
        Some(user) => Ok((StatusCode::OK, Json(UserResponse::from(user))).into_response()),
        None => Err(AppError::NotFound(format!(
            "User with email {} not found",
            payload.email
        ))),
    }
}

pub async fn create_user_handler(
    State(service): State<Arc<UserService>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let command = payload.into();
    let user = service.create_user(command).await?;
    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

pub async fn update_user_handler(
    State(service): State<Arc<UserService>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = service.update_user(id, payload).await?;
    Ok((StatusCode::OK, Json(UserResponse::from(user))))
}