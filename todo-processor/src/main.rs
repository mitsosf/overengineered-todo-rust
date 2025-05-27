use futures_util::stream::StreamExt;
use lapin::{
    options::*, 
    types::FieldTable, 
    Connection,
    ConnectionProperties,
    message::Delivery,
};
use tokio::task;
use serde::Deserialize;
use uuid::Uuid;
use dotenv::dotenv;
mod db;
mod jobs;

#[derive(Deserialize)]
struct TaskMsg {
    job_id: Uuid,
    operation: String,
    title: Option<String>,
    todo_id: Option<Uuid>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let db = db::init_db().await?;
    let addr = std::env::var("RABBITMQ_URL")?;
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;
    channel
        .queue_declare("todo_tasks", Default::default(), Default::default())
        .await?;

    let mut consumer = channel
        .basic_consume(
            "todo_tasks",
            "processor",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!(" [*] Waiting for tasks...");
    while let Some(delivery_result) = consumer.next().await {
        let delivery: Delivery = delivery_result?;
        let ack_channel = channel.clone();
        let db = db.clone();

        task::spawn(async move {
            let msg: TaskMsg = serde_json::from_slice(&delivery.data).unwrap();
            let res = match msg.operation.as_str() {
                "create" => jobs::handle_create(&db, msg.job_id, msg.title.clone().unwrap()).await,
                "delete" => jobs::handle_delete(&db, msg.job_id, msg.todo_id.unwrap()).await,
                "toggle" => jobs::handle_toggle(&db, msg.job_id, msg.todo_id.unwrap()).await,
                _        => Err(anyhow::anyhow!("unknown op")),
            };
           
            let _ = ack_channel
                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                .await;
            if let Err(e) = res {
                eprintln!("Job {} failed: {}", msg.job_id, e);
                jobs::mark_failed(&db, msg.job_id).await.ok();
            }
        });
    }
    Ok(())
}