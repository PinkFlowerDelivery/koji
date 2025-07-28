use actix_web::{post, web, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;
use crate::db::db_conn;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Expense {
    spent: u32,
    day: u8,
    month: u8,
    year: u16,
}

#[post("/expense")]
pub async fn expensehandler(req_body: web::Json<Expense>) -> impl Responder {

    let pool = db_conn().await;

    sqlx::query(
        r#"
            INSERT INTO expense (spent, day, month, year) VALUES ($1, $2, $3, $4);
        "#
    )
    .bind(req_body.spent)
    .bind(req_body.day)
    .bind(req_body.month)
    .bind(req_body.year)
    .execute(&pool).await.unwrap();

    HttpResponse::Ok().body("Data saved")
}


