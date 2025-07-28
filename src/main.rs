use actix_web::{App, HttpServer};

mod errors;
mod db;
mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    db::db_init().await;

    println!("127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(handler::expense::expensehandler)
            .service(handler::dashboard::dashboardhandler)
    })

    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
