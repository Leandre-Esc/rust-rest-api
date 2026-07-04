use std::sync::Arc;

use axum::Router;
use sqlx::PgPool;

use crate::{
    app_state::AppState,
    ping,
    users::{self, repository::PostgresUserRepository, service::UserService},
};

pub fn build_app(pool: PgPool) -> Router {
    let repository = Arc::new(PostgresUserRepository::new(pool));
    let user_service = Arc::new(UserService::new(repository));
    let state = AppState::new(user_service);

    let api_v1 = Router::new()
        .nest("/users", users::routes::router())
        .nest("/ping", ping::routes::router());

    Router::new().nest("/api/v1", api_v1).with_state(state)
}
