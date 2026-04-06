use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::users::{
    handlers::{create_user_handler, get_all_user_handler, get_user_by_email_handler},
    repository::PostgresUserRepository,
    service::UserService,
};

pub fn build_app(pool: PgPool) -> Router {
    let repository = Arc::new(PostgresUserRepository::new(pool));
    let service = Arc::new(UserService::new(repository));

    Router::new()
        .route(
            "/api/v1/user",
            post(create_user_handler).get(get_user_by_email_handler),
        )
        .route("/api/v1/users", get(get_all_user_handler))
        .with_state(service)
}
