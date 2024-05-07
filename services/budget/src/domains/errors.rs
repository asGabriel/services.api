use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Databse error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Transaction not found")]
    TransactionNotFound(Uuid),
    #[error("Account not found")]
    AccountNotFound(Uuid),
    #[error("Account has been already deleted")]
    AccountAlreadyDeleted(Uuid),
}

pub type Result<T> = std::result::Result<T, Error>;
