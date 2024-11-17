mod handler;
mod schema;
pub mod api_funcs;

use sqlx::postgres::PgPoolOptions;  // Pool, Postgres
use axum::{routing::get, Router};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database url: {}", database_url);

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5) // Adjust based on your needs
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    // creates a handler that routes 
    let app = Router::new()
        .route("/", get(handler::health_check_handler))
        .route("/database_url", get(database_url))
        .with_state(pool);

    // gets a listener for port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Starting backend...");

    // bind the address and start the server
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
