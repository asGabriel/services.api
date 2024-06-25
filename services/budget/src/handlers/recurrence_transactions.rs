use crate::domains::{errors::Result, recurrence_transactions::{CreateRecurrenceTransaction, RecurrenceTransaction}};

use super::Handler;

impl Handler {
    pub async fn create_recurrence_transaction(&self, payload: CreateRecurrenceTransaction) -> Result<RecurrenceTransaction> {
        let result = self.recurrence_transaction_repository.create_recurrence_transaction(payload).await?;

        Ok(result)
    }
}