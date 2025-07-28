use actix_web::{get, Responder, HttpResponse};
use crate::handler::expense::Expense;
use crate::db::db_conn;

#[get("/dashboard")]
pub async fn dashboardhandler() -> impl Responder {

    let pool = db_conn().await;
    let rows = sqlx::query_as::<_, Expense>("SELECT * FROM expense;").fetch_all(&pool).await.unwrap();

    let json = serde_json::to_string(&rows).unwrap();

    HttpResponse::Ok().body(format!("{}", json))
}
