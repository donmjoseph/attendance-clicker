/* ----- Table Creation Queries ----- */

pub const CREATE_QUESTIONS: &str = r#"
    CREATE TABLE IF NOT EXISTS questions (
        question_id UUID NOT NULL PRIMARY KEY,
        class_id UUID,
        class_name VARCHAR(127) NOT NULL,
        question_title VARCHAR(127) NOT NULL,
        answers INTEGER[] NOT NULL,
        correct_answer INTEGER,
        created TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )
"#;

pub const CREATE_USERS: &str = r#"
    CREATE TABLE IF NOT EXISTS users (
        user_id UUID NOT NULL PRIMARY KEY,
        name VARCHAR(127),
        email VARCHAR(127),
        password VARCHAR(127),
        level VARCHAR(127),
        registered_classes UUID[]
    )
"#;

pub const CREATE_CLASS: &str = r#"
    CREATE TABLE IF NOT EXISTS classes (
        class_id UUID NOT NULL PRIMARY KEY,
        admins UUID[],
        question_ids UUID[],
        name VARCHAR(127),
        description VARCHAR(127)
    )
"#;

pub const CREATE_LEVELS: &str = r#"
    CREATE TABLE IF NOT EXISTS levels (
        id VARCHAR(127) NOT NULL PRIMARY KEY,
        can_edit BOOL,
        can_view_basic BOOL,
        can_view_all BOOL,
        can_participate BOOL
    )
"#;

/* ----- Data Creation Queries ----- */

pub const NEW_LEVEL: &str = r#"
    INSERT INTO levels (id, can_edit, can_view_basic, can_view_all, can_participate)
    VALUES ($1, $2, $3, $4, $5)
"#;

/* ----- Data Getter Queries ----- */

pub const GET_LEVEL: &str = r#"
    SELECT id, can_edit, can_view_basic, can_view_all, can_participate FROM levels WHERE id = $1
"#;

/* ----- Data Deletion Queries ----- */

pub const DEL_LEVEL: &str = r#"
    DELETE FROM levels WHERE id = $1
"#;

/* ----- Health queries ----- */
pub const CHECK_CONNECTED: &str = r#"SELECT 1"#;
