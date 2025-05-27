use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn init_db() -> anyhow::Result<PgPool> {
    let url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;
    println!("Connected to DB");
    Ok(pool)
}