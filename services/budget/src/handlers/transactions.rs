use bigdecimal::Zero;
use finance::{status::TransactionStatus, transaction::Transaction};
use uuid::Uuid;

use crate::domains::{
    errors::{Error, Result},
    transactions::{CreateTransaction, UpdateTransaction},
};

use super::Handler;
pub mod report;

impl Handler {
    pub async fn create_transaction(&self, payload: CreateTransaction) -> Result<Transaction> {
        let total_installments = payload.installments;

        let transaction = self
            .transaction_repository
            .create_transaction(payload)
            .await?;

        if !total_installments.is_zero() {
            self.create_installment(&transaction, total_installments)
                .await?;
        }

        Ok(transaction)
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
        let result = self.get_transaction_by_id(transaction_id).await?;

        if result.is_finished() {
            return Err(Error::TransactionFinished(result.transaction_id));
        }

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
        // REFAC this route when the rules is defined
        let result = self.get_transaction_by_id(transaction_id).await?;

        self.transaction_repository
            .update_transaction_by_id(result, payload)
            .await?
            .ok_or(Error::TransactionNotFound(transaction_id))
    }

    pub async fn finish_transaction(
        &self,
        transaction_id: Uuid,
        status: TransactionStatus,
    ) -> Result<Transaction> {
        let result = self.get_transaction_by_id(transaction_id).await?;

        if result.is_finished() {
            return Err(Error::TransactionFinished(result.transaction_id));
        }

        self.transaction_repository
            .update_status(transaction_id, status)
            .await?
            .ok_or(Error::TransactionNotFound(transaction_id))
    }
}
