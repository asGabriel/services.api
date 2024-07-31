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

    pub async fn generate_recurrences(&self) -> Result<()> {
        println!("generate_Recurrences");
        let recurrences = self.recurrence_repository.list_recurrences().await?;
        let recurrences_data = recurrences.clone();
        let mut data: &Recurrence;
        let mut transaction: Transaction;
        let mut link: CreateRecurrenceLink;

        let filtered_recurrences: Vec<Uuid> = recurrences
            .iter()
            .filter(|rec| rec.is_active())
            .map(|rec| rec.recurrence_id)
            .collect();

        let recurrence_links = self
            .recurrence_repository
            .get_recurrence_link(filtered_recurrences.clone())
            .await?;

        for recurrence in filtered_recurrences {
            println!("entrou no for");
            if let Some(rec) = recurrence_links.get(&recurrence) {
                println!("letsome recurrence_links");
                if let Some(last_entry) = rec.iter().max_by_key(|r| r.due_date) {
                    println!("last entry");
                    // safe unwrap because if we find last_entry means that we
                    // have an recurrence match at recurrences_data.
                    data = recurrences_data
                        .iter()
                        .find(|r| r.recurrence_id == last_entry.recurrence_id)
                        .unwrap();

                    transaction = Recurrence::new_recurrency_transaction(data, last_entry.due_date);

                    self.transaction_repository
                        .create_transaction(transaction.clone())
                        .await?;

                    link = CreateRecurrenceLink {
                        transaction_id: transaction.transaction_id,
                        recurrence_id: data.recurrence_id,
                    };

                    self.recurrence_repository
                        .create_recurrence_link(link)
                        .await?;
                }
            } else {
                // safe unwrap because filtered_recurrences is a clone of recurrences_data
                println!("else");
                data = recurrences_data
                    .iter()
                    .find(|rec| rec.recurrence_id == recurrence)
                    .unwrap();

                transaction = Recurrence::new_recurrency_transaction(data, data.start_date);

                self.transaction_repository
                    .create_transaction(transaction.clone())
                    .await?;

                link = CreateRecurrenceLink {
                    transaction_id: transaction.transaction_id,
                    recurrence_id: data.recurrence_id,
                };

                self.recurrence_repository
                    .create_recurrence_link(link)
                    .await?;
            }
        }

        Ok(())
    }
}
