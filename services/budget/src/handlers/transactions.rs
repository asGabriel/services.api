use crate::domains::{
    errors::Result,
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
}
