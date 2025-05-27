use actix_web::{web, App, HttpServer};
mod db;
mod handlers;
mod mq;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let db_pool = db::init_db().await?;
    let mq_channel = mq::init_rabbit().await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(mq_channel.clone()))
            .configure(handlers::init_routes)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?;
    Ok(())
}
