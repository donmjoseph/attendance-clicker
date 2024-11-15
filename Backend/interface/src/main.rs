mod handler;
mod schema;

use axum::{routing::get, Router};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database url: {}", database_url);

    // creates a handler that routes 
    let app = Router::new()
        .route("/", get(handler::health_check_handler))
        .route("/database_url", get(database_url));

    // gets a listener for port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Starting backend...");

    // bind the address and start the server
    axum::serve(listener, app.into_make_service()).await.unwrap();

    println!("Backend started!");
}
