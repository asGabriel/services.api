use bigdecimal::Zero;
use uuid::Uuid;

use crate::domains::{
    errors::{Error, Result},
    transactions::{CreateTransaction, Transaction, TransactionStatus, UpdateTransaction},
};

use super::Handler;

impl Handler {
    pub async fn create_transaction(&self, payload: CreateTransaction) -> Result<Transaction> {
        let total_installments = payload.installments;

        let transaction = Transaction::from_payload(payload);

        let transaction = self
            .transaction_repository
            .create_transaction(transaction)
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
        let mut result = self.get_transaction_by_id(transaction_id).await?;

        if result.is_finished() {
            return Err(Error::TransactionFinished(result.transaction_id));
        }

        result.update(payload);

        self.transaction_repository
            .update_transaction_by_id(result)
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    use crate::repositories::{
        accounts::MockAccountRepository, installments::MockInstallmentRepository,
        recurrences::MockRecurrenceRepository, settlements::MockSettlementRepository,
        transactions::MockTransactionRepository,
    };

    #[tokio::test]
    async fn should_list_transactions() {
        let mut transaction_repository = MockTransactionRepository::new();
        let account_repository = MockAccountRepository::new();
        let installment_repository = MockInstallmentRepository::new();
        let settlement_repository = MockSettlementRepository::new();
        let recurrence_repository = MockRecurrenceRepository::new();

        transaction_repository
            .expect_list_transactions()
            .returning(move || {
                let records = vec![Transaction::default(), Transaction::default()];

                Ok(records)
            });

        let handler = Handler::new(
            Arc::new(transaction_repository),
            Arc::new(account_repository),
            Arc::new(installment_repository),
            Arc::new(settlement_repository),
            Arc::new(recurrence_repository),
        );

        let transactions = handler.list_transactions().await.unwrap();

        assert_eq!(transactions.len(), 2);
    }
}
