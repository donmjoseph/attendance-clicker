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
pub struct CreateQuestionSchema {
    pub question_id: i32,
    pub class_id: uuid::Uuid,
    pub class_name: String,
    pub question_title: String,
    pub answers: Vec<i32>,
    pub correct_answer: i32,
    pub created: chrono::NaiveDate
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessQuestionSchema {
    pub question_id: i32,
    pub class_id: uuid::Uuid
}
