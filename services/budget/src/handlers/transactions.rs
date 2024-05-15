use uuid::Uuid;

use crate::domains::{
    errors::{Error, Result},
    installments::CreateInstallment,
    transactions::{CreateTransaction, Transaction, TransactionStatus, UpdateTransaction},
};

use super::Handler;
pub mod report;

impl Handler {
    pub async fn create_transaction(&self, transaction: CreateTransaction) -> Result<Transaction> {
        let transaction = self
            .transaction_repository
            .create_transaction(transaction)
            .await?;

        if transaction.validate_installment_data().is_ok() {
            let installment_payload = CreateInstallment {
                transaction_id: transaction.transaction_id.clone(),
                amount: transaction.amount.clone(),
                due_date: transaction.due_date.clone(),
                month_reference: transaction.month_reference.clone(),
                status: transaction.status.clone(),
                year_reference: transaction.year_reference.clone(),
                recurrence_frequency: transaction.recurrence_frequency.clone(),
            };

            // SAFE unwrap because the "generate_installment" validation
            self.create_installment(installment_payload, transaction.installment_number.unwrap())
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
