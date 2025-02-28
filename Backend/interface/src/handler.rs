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
use chrono::NaiveDateTime;
use sqlx::{Row, postgres::PgPool};

use crate::schema::*;  // all schemas into scope, without a terribly long bracket list
use serde_json::json;
use uuid::Uuid;

/* Health Checks and Basic Database Functions */

pub async fn health_check_handler(State(pool): State<PgPool>) -> impl IntoResponse {
    // Log health check call
    println!("--[LOG]-- Health checker called!");

    // Select an item to see if database is connected
    if sqlx::query("SELECT 1")
            .execute(&pool)
            .await
            .is_ok() {
        ([("content-type", "json")], Json(json!({"status": "success", "message": "Backend running! Database is connected!"})))
    } else {
        ([("content-type", "json")], Json(json!({"status": "fail", "message": "Backend running! Database NOT connected."})))
    }
}

/* V1 API HANDLES */
// pub async fn create_question(State(pool): State<PgPool>, Json(body): Json<CreateQuestionSchema>) -> impl IntoResponse {
//     println!("--[LOG]-- Create question callled!");
    
//     // define the query to call
//     let query = r#"
//         INSERT INTO questions (question_id, class_id, class_name, question_title, answers, correct_answer, created)
//         VALUES ($1, $2, $3, $4, $5, $6, $7)
//     "#;

//     // make the query and store the result
//     let result = sqlx::query(query)
//         .bind(body.question_id)
//         .bind(body.class_id)
//         .bind(body.class_name)
//         .bind(body.question_title)
//         .bind(body.answers)
//         .bind(body.correct_answer)
//         .bind(body.created)
//         .execute(&pool)
//         .await;

//     // based on result, return success or error json message
//     match result {
//         Ok(_) => ([("content-type", "json")], Json(json!({"status": "success"}))),
//         Err(error) => ([("content-type", "json")], Json(json!({"status": "error", "message": error.to_string()})))
//     }
// }

// pub async fn delete_question(State(pool): State<PgPool>, Json(body): Json<AccessQuestionSchema>) -> impl IntoResponse {
//     // define query to call
//     let query = r#"
//         DELETE FROM questions
//         WHERE question_id = $1 AND class_id = $2
//     "#;

//     // make the query and store the result
//     let result = sqlx::query(query)
//         .bind(body.question_id)
//         .bind(body.class_id)
//         .execute(&pool)
//         .await;

//     // based on result, return success or error json message
//     match result {
//         Ok(_) => ([("content-type", "json")], Json(json!({"status": "success"}))),
//         Err(error) => ([("content-type", "json")], Json(json!({"status": "fail", "message": error.to_string()})))
//     }
// }

// pub async fn get_question_info(State(pool): State<PgPool>, Json(body): Json<AccessQuestionSchema>) -> impl IntoResponse {
//     println!("--[LOG]-- Get Question Called!");
    
//     // define query to call
//     let query = r#"
//         SELECT * FROM questions
//         WHERE question_id = $1
//     "#;

//     // make the query and store the result
//     let result = sqlx::query(query)
//         .bind(body.question_id)
//         .fetch_one(&pool)
//         .await;

//     // based on the result, return success with the data or error json message
//     match result {
//         Ok(row) => {
//             let question = CreateQuestionSchema {
//                 question_id: row.get::<i32, _>("question_id"),
//                 class_id: row.get::<Uuid, _>("class_id"),
//                 class_name: row.get::<String, _>("class_name"),
//                 question_title: row.get::<String, _>("question_title"),
//                 answers: row.get::<Vec<i32>, _>("answers"),
//                 correct_answer: row.get::<i32, _>("correct_answer"),
//                 created: row.get::<NaiveDateTime, _>("created"),
//             };
//             ([("content-type", "json")], Json(json!({"status": "success", "question": question})))
//         },
//         Err(error) => ([("content-type", "json")], Json(json!({"status": "fail", "message": error.to_string()})))
//     }
// }
