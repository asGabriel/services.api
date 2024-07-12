use crate::domains::{errors::Result, recurrences::{CreateRecurrence, Recurrence}};

use super::Handler;

impl Handler {
    pub async fn list_recurrences(&self) -> Result<Vec<Recurrence>> {
        let recurrences = self.recurrence_repository.list_recurrences().await?;

        Ok(recurrences)
    }

    pub async fn create_recurrence(&self, payload: CreateRecurrence) -> Result<Recurrence> {
        let recurrence = self.recurrence_repository.create_recurrence(Recurrence::new_from_payload(payload)).await?;

        Ok(recurrence)
    }
}
