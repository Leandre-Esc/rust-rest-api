use axum::{routing::post, Router};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use dotenvy::dotenv;
use infra::postgres::PostgresRepository;
use application::user::UserService;
use web::user::create_user_handler;

mod domain;
mod application;
mod infra;
mod web;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL muse be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Impossible de se connecter à Postgres");

    let repo = Arc::new(PostgresRepository::new(pool));
    let service = Arc::new(UserService::new(repo));

    let app = Router::new()
        .route("/api/v1/user", post(create_user_handler))
        .with_state(service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("✅ Server running on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}