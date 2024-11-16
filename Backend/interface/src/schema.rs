use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StudentAccessSchema {
    pub id: i64,
    pub name: String,
    pub inscure_password: String
}
