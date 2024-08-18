use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn establish_connection(database_url: &str) -> Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("Failed to create pool.")?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tb_app (
            id VARCHAR PRIMARY KEY,
            md TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .context("Failed to create table.")?;

    Ok(pool)
}
