use sqlx::postgres::PgPool;
use serde_json::json;
use axum::{response::IntoResponse, Json, extract::State};

pub mod GET;
pub mod POST;

pub mod manage;
pub mod users_table;
pub mod questions_table;
pub mod classes_table;
pub mod levels_table;

pub mod query_consts;

pub async fn startup_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    questions_table::setup_table(pool).await?;
    users_table::setup_table(pool).await?;
    classes_table::setup_table(pool).await?;
    levels_table::setup_table(pool).await?;

    Ok(())
}

pub async fn check_connected(State(pool): State<PgPool>) -> impl IntoResponse {
    // Log health check call
    println!("--[LOG]-- Health checker called!!!!");

    // Select an item to see if database is connected
    if sqlx::query(query_consts::CHECK_CONNECTED)
            .execute(&pool)
            .await
            .is_ok() {
        ([("content-type", "json")], Json(json!({"status": "success", "message": "Backend running! Database is connected!"})))
    } else {
        ([("content-type", "json")], Json(json!({"status": "fail", "message": "Backend running! Database NOT connected."})))
    }
}