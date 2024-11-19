/*
All the logic
*/

// use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    // http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::postgres::PgPool;

use crate::schema::CreateClassSchema;
use serde_json::json;

/* Health Checks and Basic Database Functions */

pub async fn health_check_handler() -> impl IntoResponse {
    println!("Health checker called!");

    const MESSAGE: &str = "Backend is running! Database health unknown.";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn setup_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    let create_polls_table = r#"
        CREATE TABLE IF NOT EXISTS questions (
            question_id INTEGER NOT NULL PRIMARY KEY,
            class_id UUID,
            class_name VARCHAR(127) NOT NULL,
            question_title VARCHAR(127) NOT NULL,
            answers INTEGER[] NOT NULL,
            correct_answer INTEGER,
            created TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    "#;

    sqlx::query(create_polls_table).execute(pool).await?;

    // id SERIAL PRIMARY KEY,
    // name TEXT NOT NULL,
    // created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP

    Ok(())
}

/* V1 API HANDLES */
pub async fn create_question(State(pool): State<PgPool>, Json(body): Json<CreateClassSchema>) -> impl IntoResponse {
    // define the query to call
    let query = r#"
        INSERT INTO questions (question_id, class_id, class_name, question_title, answers, correct_answer, created)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
    "#;

    // make the query and store the result
    let result = sqlx::query(query)
        .bind(body.question_id)
        .bind(body.class_id)
        .bind(body.class_name)
        .bind(body.question_title)
        .bind(body.answers)
        .bind(body.correct_answer)
        .bind(body.created)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Json(json!({"status": "success"})),
        Err(error) => Json(json!({"status": "error", "message": error.to_string()}))
    }
}
