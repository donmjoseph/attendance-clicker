use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;
use sqlx::{postgres::PgPool, Row};

use crate::{api::query_consts, schema};

// Create the table if it doesn't exist yet
pub async fn setup_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(query_consts::CREATE_LEVELS).execute(pool).await?;

    Ok(())
}

// pub async fn level_exists(State(pool): State<PgPool>, Json(body): Json<schema::AccessLevelSchema>) -> Result<bool, sqlx::Error> {
//     let result: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM Levels WHERE id=$1")
//         .bind(body.id)
//         .fetch_one(&pool)
//         .await?;

//     Ok(result.0)
// }

// add a new row with the given level data
pub async fn create_level(State(pool): State<PgPool>, Json(body): Json<schema::LevelsSchema>) -> impl IntoResponse {
    let result = sqlx::query(query_consts::NEW_LEVEL)
        .bind(body.id)
        .bind(body.can_edit)
        .bind(body.can_view_basic)
        .bind(body.can_view_all)
        .bind(body.can_participate)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => ([("content-type", "json")], Json(json!({"status": "success"}))),
        Err(error) => ([("content-type", "json")], Json(json!({"status": "error", "message": error.to_string()})))
    }
}

// get the info about a given level's id, if it exists
pub async fn get_level_info(State(pool): State<PgPool>, Json(body): Json<schema::AccessLevelSchema>) -> impl IntoResponse {
    let result = sqlx::query(query_consts::GET_LEVEL)
        .bind(body.id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(row) => {
            let level = schema::LevelsSchema {
                id: row.get::<String, _>("id"),
                can_edit: row.get::<bool, _>("can_edit"),
                can_view_basic: row.get::<bool, _>("can_view_basic"),
                can_view_all: row.get::<bool, _>("can_view_all"),
                can_participate: row.get::<bool, _>("can_participate")
            };
            ([("content-type", "json")], Json(json!({"status": "success", "level": level})))
        },
        Err(error) => ([("content-type", "json")], Json(json!({"status": "error", "message": error.to_string()})))
    }
}

// delete a level matching the given id, if it exists
pub async fn delete_level(State(pool): State<PgPool>, Json(body): Json<schema::AccessLevelSchema>) -> impl IntoResponse {
    let result = sqlx::query(query_consts::DEL_LEVEL)
        .bind(body.id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => ([("content-type", "json")], Json(json!({"status": "success"}))),
        Err(error) => ([("content-type", "json")], Json(json!({"status": "error", "message": error.to_string()})))
    }
}