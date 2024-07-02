use chrono::{Local, NaiveDate};
use finance::transaction::Transaction;

use crate::domains::{
    errors::Result,
    recurrence_transactions::{CreateRecurrenceTransaction, RecurrenceTransaction},
};

use super::Handler;

impl Handler {
    pub async fn list_recurrence_transactions(&self) -> Result<Vec<RecurrenceTransaction>> {
        let result = self
            .recurrence_transaction_repository
            .list_recurrence_transactions()
            .await?;

        Ok(result)
    }

    pub async fn create_recurrence_transaction(
        &self,
        payload: CreateRecurrenceTransaction,
    ) -> Result<RecurrenceTransaction> {
        let result = self
            .recurrence_transaction_repository
            .create_recurrence_transaction(payload)
            .await?;

        Ok(result)
    }

    pub async fn generate_recurrence_transaction(&self) -> Result<Vec<Transaction>> {
        let mut recurrence_transactions = self.list_recurrence_transactions().await?;
        let generated_transactions = self
            .recurrence_transaction_repository
            .list_generated_transactions_from_recurrence()
            .await?;
        let now = Local::now().date_naive();

        recurrence_transactions = recurrence_transactions
            .into_iter()
            .filter(|rec_transaction| rec_transaction.is_active == true)
            .collect();

        for rec_transaction in recurrence_transactions {}

        unimplemented!();
    }
}
