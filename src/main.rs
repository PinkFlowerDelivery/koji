use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod errors;
mod db;
mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    db::db_init().await;
    let addr = env::var("ADDR").unwrap();
    let port: u16 = env::var("PORT").unwrap().parse().unwrap();

    println!("{}:{}", addr, port);
    HttpServer::new(|| {
        App::new()
            .service(handler::expense::expensehandler)
            .service(handler::dashboard::dashboardhandler)
    })
    .bind((addr, port))?
    .run()
    .await
}
