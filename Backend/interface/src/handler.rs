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

use crate::schema::*;  // all schemas into scope, without a terribly long bracket list
use serde_json::json;

/* Health Checks and Basic Database Functions */

pub async fn health_check_handler(State(pool): State<PgPool>) -> impl IntoResponse {
    // Log health check call
    println!("--[LOG]-- Health checker called!");

    // Select an item to see if database is connected
    if sqlx::query("SELECT 1")
            .execute(&pool)
            .await
            .is_ok() {
        Json(json!({"status": "success", "message": "Backend running! Database is connected!"}))
    } else {
        Json(json!({"stauts": "fail", "message": "Backend was running! But database is not connected."}))
    }
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

    Ok(())
}

/* V1 API HANDLES */
pub async fn create_question(State(pool): State<PgPool>, Json(body): Json<CreateQuestionSchema>) -> impl IntoResponse {
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

    // based on result, return success or error json message
    match result {
        Ok(_) => Json(json!({"status": "success"})),
        Err(error) => Json(json!({"status": "error", "message": error.to_string()}))
    }
}

pub async fn delete_question(State(pool): State<PgPool>, Json(body): Json<AccessQuestionSchema>) -> impl IntoResponse {
    // define query to call
    let query = r#"
        DELETE FROM questions
        WHERE question_id = $1 AND class_id = $2
    "#;

    // make the query and store the result
    let result = sqlx::query(query)
        .bind(body.question_id)
        .bind(body.class_id)
        .execute(&pool)
        .await;

    // based on result, return success or error json message
    match result {
        Ok(_) => Json(json!({"status": "success"})),
        Err(error) => Json(json!({"status": "fail", "message": error.to_string()}))
    }
}

pub async fn get_question_info(State(pool): State<PgPool>, Json(body): Json<AccessQuestionSchema>) -> impl IntoResponse {
    Json(json!({"status": "success"}))
}