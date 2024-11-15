/*
All the logic
*/

// use std::sync::Arc;

use axum::{
    // extract::{Path, Query, State},
    // http::StatusCode,
    response::IntoResponse,
    Json,
};
// use serde_json::json;

pub async fn health_check_handler() -> impl IntoResponse {
    println!("Health checker called!");

    const MESSAGE: &str = "Backend is running! Database health unknown.";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}