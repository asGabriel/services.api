use chrono::Utc;
use uuid::Uuid;

use crate::domains::{
    errors::{Error, Result},
    recurrences::{CreateRecurrence, CreateRecurrenceLink, Recurrence, UpdateRecurrence},
    transactions::Transaction,
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

    // REFAC: change logic to financial_plan
    pub async fn generate_recurrences(&self) -> Result<()> {
        todo!()
        // let recurrences = self.recurrence_repository.list_recurrences().await?;

        // let active_recurrences: Vec<&Recurrence> =
        //     recurrences.iter().filter(|r| r.is_active()).collect();
        // let recurrence_ids: Vec<Uuid> =
        //     active_recurrences.iter().map(|r| r.recurrence_id).collect();

        // let references = self
        //     .recurrence_repository
        //     .get_recurrence_link(recurrence_ids)
        //     .await?;

        // for recurrence in active_recurrences {
        //     let last_recurrency = references
        //         .get(&recurrence.recurrence_id)
        //         .and_then(|r| r.iter().max_by_key(|item| item.due_date));

        //     let next_due_date = match last_recurrency {
        //         Some(r) => recurrence.get_next_date_from_frequency(r.due_date),
        //         None => recurrence.get_next_date_from_frequency(recurrence.start_date),
        //     };

        //     let today = Utc::now().date_naive();
        //     if next_due_date <= today {
        //         let payload = recurrence.new_recurrency_transaction(next_due_date);

        //         let transaction = self
        //             .transaction_repository
        //             .create_transaction(Transaction::from_payload(payload))
        //             .await?;

        //         let link = CreateRecurrenceLink {
        //             recurrence_id: recurrence.recurrence_id,
        //             transaction_id: transaction.transaction_id,
        //         };

        //         self.recurrence_repository
        //             .create_recurrence_link(link)
        //             .await?;
        //     }
        // }

        // Ok(())
    }
}
