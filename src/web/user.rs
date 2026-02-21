use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use crate::application::user::UserService;
use crate::web::dto::{CreateUserRequest, UserResponse};

pub async fn create_user_handler(
    State(service): State<Arc<UserService>>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    // Transformation DTO -> Commande Domaine
    let command = payload.into();

    match service.create_user(command).await {
        Ok(user) => {
            let response = UserResponse {
                id: user.id,
                full_name: format!("{} {}", user.first_name.unwrap_or_default(), user.last_name.unwrap_or_default()),
                username: user.username,
                email: user.email,
                created_at: user.created_at,
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e)).into_response(),
    }
}