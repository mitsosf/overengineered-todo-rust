use actix_web::{web, App, HttpServer};
mod db;
mod handlers;
mod mq;
use dotenv::dotenv;
use actix_cors::Cors;
use actix_web::http::header;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let db_pool = db::init_db().await?;
    let mq_channel = mq::init_rabbit().await?;

    HttpServer::new(move || {
        // configure CORS to allow only specific origins
        let cors = Cors::default()
            .allowed_origin("https://overengineered-todos.frangiadakis.com")
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://localhost:6967")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(mq_channel.clone()))
            .configure(handlers::init_routes)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?;
    Ok(())
}
