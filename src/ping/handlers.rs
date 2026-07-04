use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct PongResponse {
    message: String,
}

pub async fn pong() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(PongResponse {
            message: "pong".to_string(),
        }),
    )
}
