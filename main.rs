use actix_web::{web, App, HttpServer};
use sqlx::mysql::MySqlPool;
use std::io;

mod handlers;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let database_url = "mysql://user:password@localhost/dbname";
    let pool = MySqlPool::connect(&database_url).await.expect("Failed to connect to the database");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(handlers::buy_product)
            .service(handlers::sell_product)
            .service(handlers::get_delivery_status)
            .service(handlers::get_product_detail)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
