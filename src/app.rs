use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use axum::routing::patch;
use sqlx::PgPool;

use crate::users::{
    handlers::{create_user_handler, get_all_user_handler, get_user_by_email_handler},
    repository::PostgresUserRepository,
    service::UserService,
};
use crate::users::handlers::update_user_handler;

pub fn build_app(pool: PgPool) -> Router {
    let repository = Arc::new(PostgresUserRepository::new(pool));
    let service = Arc::new(UserService::new(repository));

    Router::new()
        .route("/api/v1/users", get(get_all_user_handler))
        .route("/api/v1/users", get(get_user_by_email_handler))
        .route("/api/v1/users", post(create_user_handler))
        .route("/api/v1/users/:id", patch(update_user_handler))
        .with_state(service)
}
