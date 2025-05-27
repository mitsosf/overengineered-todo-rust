use lapin::{Connection, ConnectionProperties, Channel};
use std::env;

pub async fn init_rabbit() -> anyhow::Result<Channel> {
    let addr = env::var("RABBITMQ_URL")?;
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;
    // ensure our queue exists
    channel.queue_declare("todo_tasks", Default::default(), Default::default()).await?;
    Ok(channel)
}