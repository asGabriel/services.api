use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Databse error")]
    DatabaseError(#[from] sqlx::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
