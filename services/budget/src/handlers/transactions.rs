use uuid::Uuid;

use crate::domains::{
    errors::{Error, Result},
    transactions::{CreateTransactionDto, Transaction},
};

use super::Handler;

impl Handler {
    pub async fn create_transaction(
        &self,
        transaction: CreateTransactionDto,
    ) -> Result<Transaction> {
        self.transactions_repository
            .create_transaction(transaction)
            .await
    }

    pub async fn list_transactions(&self) -> Result<Vec<Transaction>> {
        self.transactions_repository.list_transactions().await
    }

    pub async fn get_transaction_by_id(&self, transaction_id: Uuid) -> Result<Transaction> {
        self.transactions_repository
            .get_transaction_by_id(transaction_id)
            .await?
            .ok_or(Error::TransactionNotFound(transaction_id))
    }
}
