use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Db Error")]
    DB(#[from] sqlx::Error),

    #[error("IO Error")]
    IO(#[from] std::io::Error),

    #[error("Actix Error")]
    Actix(#[from] actix_web::error::Error),

}
