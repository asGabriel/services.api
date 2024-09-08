use finance_domains::Entry;
use uuid::Uuid;

use crate::domains::{
    entries::EntryPayload,
    errors::{Error, Result},
};

use super::Handler;

impl Handler {
    pub async fn list_entries(&self) -> Result<Vec<Entry>> {
        self.entries_repository.list_entries().await
    }

    pub async fn get_entry_by_id(&self, entry_id: Uuid) -> Result<Entry> {
        self.entries_repository
            .get_entry_by_id(entry_id)
            .await?
            .ok_or(Error::EntryNotFound(entry_id))
    }

    pub async fn create_entry(&self, payload: EntryPayload) -> Result<Entry> {
        let entry = payload.into();

        self.entries_repository.create_entry(entry).await
    }
}
