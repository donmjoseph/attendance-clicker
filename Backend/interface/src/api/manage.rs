use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use sqlx::postgres::PgPool;
use serde_json::json;

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