use serde::{Deserialize, Serialize};
use chrono;
use uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct StudentAccessSchema {
    pub id: i64,
    pub name: String,
    pub inscure_password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateClassSchema {
    question_id: i32,
    class_id: uuid::Uuid,
    class_name: String,
    question_title: String,
    answers: Vec<i32>,
    correct_answer: i32,
    created: chrono::NaiveDate
}