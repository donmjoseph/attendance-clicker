mod handler;
mod schema;
pub mod api_funcs;

use sqlx::postgres::PgPoolOptions;  // Pool, Postgres
use axum::{http::Method, routing::{get, post}, Router};
use dotenv::dotenv;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("--[INFO]-- Rust backend `backend-interface-1` starting, dotenv() OK");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("--[INFO]-- database url found: {}", database_url);

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5) // Adjust based on your needs
        .connect(&database_url)
        .await
        .expect("--[ERROR]-- Failed to create database pool");

    println!("--[INFO]-- Successfully connected to database, and created pool");

    // setup database with arbitrary, hard-coded data. Handle error if fail.
    if let Err(e) = handler::setup_database(&pool).await {
        eprintln!("--[ERROR]-- Database setup failed. Error is as follows: {}", e);
        std::process::exit(1);
    }

    println!("--[SUCCESS]-- Database setup succeeded!");

    // create a CORS layer to allow request from any origin
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any);

    // creates a handler that routes 
    let app = Router::new()
        .route("/", get(handler::health_check_handler))
        .route("/database_url", get(database_url))
        .route("/create_question", post(handler::create_question))
        .route("/delete_question", post(handler::delete_question))
        .route("/get_question", get(handler::get_question_info))
        .layer(cors)
        .with_state(pool);

    // make address for listener, then make listener on that address and port
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    println!("--[SUCCESS]-- Router made, listener bound to `{}`, serving app via Axum...", addr);

    // bind the address and start the server
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
