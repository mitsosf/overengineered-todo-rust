use sqlx::{Transaction, Executor, PgPool};
use uuid::Uuid;

pub async fn handle_create(db: &PgPool, job_id: Uuid, todo_id: Uuid, title: String) -> anyhow::Result<()> {
    println!("Processing create job: {}", job_id);
    let mut tx: Transaction<'_, _> = db.begin().await?;

    tx.execute(
        sqlx::query(
            "INSERT INTO todos (id, title) VALUES ($1, $2)"
        )
            .bind(todo_id)
            .bind(title)
    )
        .await?;

    tx.execute(
        sqlx::query(
            "UPDATE jobs SET status = 'completed' WHERE id = $1"
        )
            .bind(job_id)
    )
        .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn handle_delete(db: &PgPool, job_id: Uuid, todo_id: Uuid) -> anyhow::Result<()> {
    println!("Processing delete job: {}", job_id);
    let mut tx = db.begin().await?;

    tx.execute(
        sqlx::query("DELETE FROM todos WHERE id=$1").bind(todo_id)
    )
        .await?;

    tx.execute(
        sqlx::query(
            "UPDATE jobs SET status = 'completed' WHERE id = $1"
        )
            .bind(job_id)
    )
        .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn handle_toggle(
    db: &PgPool,
    job_id: Uuid,
    todo_id: Uuid,
) -> anyhow::Result<()> {
    println!("Processing toggle job: {}", job_id);
    
    let mut tx = db.begin().await?;
    let result = tx.execute(
        sqlx::query(
            "UPDATE todos 
                 SET completed = NOT completed 
                 WHERE id = $1"
        ).bind(todo_id)
    )
        .await?;
    if result.rows_affected() == 0 {
        return Err(anyhow::anyhow!("Todo {} not found", todo_id));
    }
    
    tx.execute(
        sqlx::query(
            "UPDATE jobs SET status = 'completed' WHERE id = $1"
        ).bind(job_id)
    ).await?;
    
    tx.commit().await?;
    Ok(())
}

pub async fn mark_failed(db: &PgPool, job_id: Uuid) -> anyhow::Result<()> {
    sqlx::query("UPDATE jobs SET status='failed' WHERE id=$1")
        .bind(job_id)
        .execute(db).await?;
    Ok(())
}