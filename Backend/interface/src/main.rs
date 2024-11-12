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
        .route("/", get(handle_hello))
        .route("/database_url", get(database_url))
        .route("/goodbye", get(handle_goodbye))
        .route("/ea-sports", get(in_the_game)); // i have brainrot

    // gets a listener for port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Starting backend... once more");

    // bind the address and start the server
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn handle_hello() -> String {
    "Hello, World!!!!!!".to_string()
}

async fn handle_goodbye() ->  String {
    "Goodbye, World!".to_string()
}

async fn in_the_game() -> String {
    "It's in the game".to_string()
}