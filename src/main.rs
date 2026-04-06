use dotenvy::dotenv;
use rust_rest_api::app::build_app;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL muse be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Connection to database failed");

    let app = build_app(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("✅ Server running on http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}
