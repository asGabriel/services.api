use http_problems::{Error, Result};
use uuid::Uuid;

use crate::domains::entries::{Entry, EntryPayload};

use super::Handler;

impl Handler {
    pub async fn list_entries(&self) -> Result<Vec<Entry>> {
        self.entries_repository.list_entries().await
    }

    pub async fn get_entry_by_id(&self, entry_id: Uuid) -> Result<Entry> {
        self.entries_repository
            .get_entry_by_id(entry_id)
            .await?
            .ok_or(Error::NotFoundError(format!(
                "Entry id {entry_id} not found.",
            )))
    }

    pub async fn create_entry(&self, payload: EntryPayload) -> Result<Entry> {
        let entry = payload.into();

        self.entries_repository.create_entry(entry).await
    }

    pub async fn list_entries_by_invoice_id(&self, invoice_id: Uuid) -> Result<Vec<Entry>> {
        self.entries_repository
            .get_entries_by_invoice_id(invoice_id)
            .await
    }
}
