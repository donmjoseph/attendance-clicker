pub const USERS_CREATE: &str = r#"
    CREATE TABLE IF NOT EXISTS Users (
        user_id UUID NOT NULL PRIMARY KEY,
        name VARCHAR(127)
    )
"#;

pub const QUESTIONS_CREATE: &str = r#"
    CREATE TABLE IF NOT EXISTS Questions (
        question_id UUID NOT NULL PRIMARY KEY,
        class_id UUID,
        class_name VARCHAR(127) NOT NULL,
        question_title VARCHAR(127) NOT NULL,
        answers INTEGER[] NOT NULL,
        correct_answer INTEGER,
        created TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )
"#;
