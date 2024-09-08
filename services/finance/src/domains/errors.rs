use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Databse error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Invoice not found")]
    InvoiceNotFound(Uuid),
    #[error("Entry not found")]
    EntryNotFound(Uuid),
}

pub type Result<T> = std::result::Result<T, Error>;
