use uuid::Uuid;

use crate::domains::{
    errors::{Error, Result},
    recurrences::{CreateRecurrence, Recurrence, UpdateRecurrence},
};

use super::Handler;

impl Handler {
    pub async fn list_recurrences(&self) -> Result<Vec<Recurrence>> {
        let recurrences = self.recurrence_repository.list_recurrences().await?;

        Ok(recurrences)
    }

    pub async fn create_recurrence(&self, payload: CreateRecurrence) -> Result<Recurrence> {
        let recurrence = self
            .recurrence_repository
            .create_recurrence(Recurrence::new_from_payload(payload))
            .await?;

        Ok(recurrence)
    }

    pub async fn get_recurrence_by_id(&self, recurrence_id: Uuid) -> Result<Recurrence> {
        self.recurrence_repository
            .get_recurrence_by_id(recurrence_id)
            .await?
            .ok_or(Error::RecurrenceNotFound(recurrence_id))
    }

    pub async fn update_recurrence(
        &self,
        recurrence_id: Uuid,
        payload: UpdateRecurrence,
    ) -> Result<Recurrence> {
        let mut recurrence = self.get_recurrence_by_id(recurrence_id).await?;

        recurrence.update(payload);

        let result = self
            .recurrence_repository
            .update_recurrence(recurrence)
            .await?
            .ok_or(Error::RecurrenceNotFound(recurrence_id))?;

        Ok(result)
    }
}
