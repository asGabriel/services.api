use uuid::Uuid;

use crate::domains::{
    errors::{Error, Result},
    transactions::{CreateTransaction, Transaction, UpdateTransaction},
};

use super::Handler;

impl Handler {
    pub async fn create_transaction(&self, transaction: CreateTransaction) -> Result<Transaction> {
        self.transaction_repository
            .create_transaction(transaction)
            .await
    }

    pub async fn list_transactions(&self) -> Result<Vec<Transaction>> {
        self.transaction_repository.list_transactions().await
    }

    pub async fn get_transaction_by_id(&self, transaction_id: Uuid) -> Result<Transaction> {
        self.transaction_repository
            .get_transaction_by_id(transaction_id)
            .await?
            .ok_or(Error::TransactionNotFound(transaction_id))
    }

    pub async fn delete_transaction_by_id(&self, transaction_id: Uuid) -> Result<Transaction> {
        self.get_transaction_by_id(transaction_id).await?;

        self.transaction_repository
            .delete_transaction_by_id(transaction_id)
            .await?
            .ok_or(Error::TransactionNotFound(transaction_id))
    }

    pub async fn update_transaction_by_id(
        &self,
        transaction_id: Uuid,
        payload: UpdateTransaction,
    ) -> Result<Transaction> {
        let result = self.get_transaction_by_id(transaction_id).await?;

        self.transaction_repository
            .update_transaction_by_id(result, payload)
            .await?
            .ok_or(Error::TransactionNotFound(transaction_id))
    }
}
