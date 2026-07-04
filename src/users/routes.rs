use axum::{
    Router,
    routing::{get, patch},
};

use crate::{
    app_state::AppState,
    users::handlers::{
        create_user_handler, delete_user_handler, get_all_user_handler, get_user_by_email_handler,
        update_user_handler,
    },
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_user_handler).post(create_user_handler))
        .route("/search", get(get_user_by_email_handler))
        .route(
            "/{id}",
            patch(update_user_handler).delete(delete_user_handler),
        )
}
