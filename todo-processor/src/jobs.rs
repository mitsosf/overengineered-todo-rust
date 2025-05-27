use sqlx::{Transaction, Executor, PgPool};
use uuid::Uuid;

pub async fn handle_create(db: &PgPool, job_id: Uuid, title: String) -> anyhow::Result<()> {
    println!("Processing create job: {}", job_id);
    let todo_id = Uuid::new_v4();

    // begin() gives you an owned Transaction<_, Postgres>
    let mut tx: Transaction<'_, _> = db.begin().await?;

    tx.execute(
        sqlx::query!(
            "INSERT INTO todos (id, title) VALUES ($1, $2)",
            todo_id,
            title
        )
    )
        .await?;

    tx.execute(
        sqlx::query!(
            "INSERT INTO jobs (id, todo_id, operation, status) \
             VALUES ($1, $2, 'create', 'completed')",
            job_id,
            todo_id
        )
    )
        .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn handle_delete(db: &PgPool, job_id: Uuid, todo_id: Uuid) -> anyhow::Result<()> {
    println!("Processing delete job: {}", job_id);
    let mut tx = db.begin().await?;

    tx.execute(
        sqlx::query!("DELETE FROM todos WHERE id=$1", todo_id)
    )
        .await?;

    tx.execute(
        sqlx::query!(
            "INSERT INTO jobs (id, todo_id, operation, status) \
             VALUES ($1, $2, 'delete', 'completed')",
            job_id,
            todo_id
        )
    )
        .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn mark_failed(db: &PgPool, job_id: Uuid) -> anyhow::Result<()> {
    sqlx::query!("UPDATE jobs SET status='failed' WHERE id=$1", job_id)
        .execute(db).await?;
    Ok(())
}