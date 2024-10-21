use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // creates a handler that routes 
    let app = Router::new()
        .route("/", get(handle_hello))
        .route("/goodbye", get(handle_goodbye))
        .route("/ea-sports", get(in_the_game)); // i have brainrot

    // gets a listener for port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handle_hello() -> String {
    "Hello, World!".to_string()
}

async fn handle_goodbye() ->  String {
    "Goodbye, World!".to_string()
}

async fn in_the_game() -> String {
    "It's in the game".to_string()
}