use sqlx::postgres::PgPool;

use crate::api::query_consts;

pub async fn setup_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(query_consts::CREATE_CLASS).execute(pool).await?;

    Ok(())
}