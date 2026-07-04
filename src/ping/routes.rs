use axum::{Router, routing::get};

use crate::{app_state::AppState, ping::handlers::pong};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(pong))
}
