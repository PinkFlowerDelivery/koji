use std::str::FromStr;

use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, Pool, Sqlite};

pub async fn db_conn() -> Pool<Sqlite> {
    let db_url = "sqlite://data.db";

    SqlitePoolOptions::new()
        .max_connections(10)
        .idle_timeout(std::time::Duration::from_secs(30))
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect_with(
            SqliteConnectOptions::from_str(db_url)
                .unwrap()
                .create_if_missing(true)
                .journal_mode(sqlx::sqlite::SqliteJournalMode::Delete),
        ).await.unwrap()   
}

pub async fn db_init() {
    let pool = db_conn().await;
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS expense (
            spent INTEGER,
            day INTEGER,
            month INTEGER,
            year INTEGER
        )
        "#
    ).execute(&pool).await.unwrap();

    pool.close().await;
}
