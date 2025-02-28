use sqlx::postgres::PgPool;

pub mod GET;
pub mod POST;

pub mod manage;
pub mod users_table;
pub mod questions_table;

pub mod query_consts;

pub async fn startup_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    questions_table::setup_table(pool).await?;
    users_table::setup_table(pool).await?;

    Ok(())
}
