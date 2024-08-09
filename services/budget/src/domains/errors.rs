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
    #[error("Transaction has been already finished")]
    TransactionFinished(Uuid),
    #[error("Account not found")]
    InstallmentNotFound(Uuid),
    #[error("Installment has been already finished")]
    InstallmentFinished(Uuid),
    #[error("Recurrence not found")]
    RecurrenceNotFound(Uuid),
    #[error("Financial plan not found")]
    FinancialPlanNotFound(Uuid),
}

pub type Result<T> = std::result::Result<T, Error>;
