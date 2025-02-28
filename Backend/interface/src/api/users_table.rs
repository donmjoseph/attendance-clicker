use sqlx::postgres::PgPool;

use crate::api::query_consts;

pub async fn setup_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(query_consts::USERS_CREATE).execute(pool).await?;

    Ok(())
}