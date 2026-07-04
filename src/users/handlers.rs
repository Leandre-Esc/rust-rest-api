use crate::app_state::AppState;
use crate::shared::error::AppError;
use crate::users::dto::{CreateUserRequest, GetUserByEmailRequest, UserResponse, UsersResponse};
use crate::users::dto::{UpdateUserRequest, UserDeleteResponse};
use axum::extract::Path;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use uuid::Uuid;

pub async fn get_all_user_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let users = state.user_service.get_all_user().await?;

    Ok((StatusCode::OK, Json(UsersResponse::from(users))))
}

pub async fn get_user_by_email_handler(
    State(state): State<AppState>,
    Json(payload): Json<GetUserByEmailRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.get_user_by_email(&payload.email).await?;

    match user {
        Some(user) => Ok((StatusCode::OK, Json(UserResponse::from(user))).into_response()),
        None => Err(AppError::NotFound(format!(
            "User with email {} not found",
            payload.email
        ))),
    }
}

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let command = payload.into();
    let user = state.user_service.create_user(command).await?;
    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

pub async fn update_user_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.user_service.update_user(id, payload).await?;
    Ok((StatusCode::OK, Json(UserResponse::from(user))))
}

pub async fn delete_user_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let id = state.user_service.delete_user(id).await?;
    Ok((StatusCode::OK, Json(UserDeleteResponse { id })))
}
