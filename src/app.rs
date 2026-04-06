use std::sync::Arc;

use axum::routing::patch;
use axum::{
    routing::get,
    Router,
};
use sqlx::PgPool;

use crate::users::{
    handlers::{create_user_handler, get_all_user_handler, get_user_by_email_handler, update_user_handler},
    repository::PostgresUserRepository,
    service::UserService,
};

pub fn build_app(pool: PgPool) -> Router {
    let repository = Arc::new(PostgresUserRepository::new(pool));
    let service = Arc::new(UserService::new(repository));

    let users_routes =  Router::new()
        .route("/", get(get_all_user_handler).post(create_user_handler))
        .route("/search", get(get_user_by_email_handler))
        .route("/{id}", patch(update_user_handler));

    Router::new()
        .nest("/api/v1/users", users_routes)
        .with_state(service)
}
