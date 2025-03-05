use serde::{Deserialize, Serialize};
use chrono;
use uuid::Uuid;

/* ----- Questions ----- */
#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionSchema {
    pub question_id: Uuid,
    pub class_id: Uuid,
    pub title: String,
    pub description: String,
    pub answers: Vec<i32>,
    pub correct: i32,
    pub created: chrono::NaiveDateTime
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessQuestionSchema {
    pub question_id: Uuid
}

/* ----- Class ----- */
#[derive(Serialize, Deserialize, Debug)]
pub struct ClassSchema {
    pub class_id: Uuid,
    pub admins: Vec<Uuid>,
    pub question_ds: Vec<Uuid>,
    pub name: String,
    pub description: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessClassSchema {
    pub class_id: Uuid
}

/* ----- User ----- */
#[derive(Serialize, Deserialize, Debug)]
pub struct UserSchema {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub pwd: String,
    pub level: String,
    pub registered_classes: Vec<Uuid>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessUserSchema {
    pub user_id: Uuid
}

/* ----- Levels ----- */
#[derive(Serialize, Deserialize, Debug)]
pub struct LevelsSchema {
    pub id: String,
    pub can_edit: bool,
    pub can_view_basic: bool,
    pub can_view_all: bool,
    pub can_participate: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessLevelSchema {
    pub id: String
}
