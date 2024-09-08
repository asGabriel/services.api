use bigdecimal::BigDecimal;
use chrono::{NaiveDate, Utc};
use finance_domains::entries::{Entry, EntryStatus, EntryType};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryPayload {
    pub invoice_id: Uuid,
    pub entry_type: EntryType,
    pub description: String,
    pub value: BigDecimal,
    pub due_date: NaiveDate,
    pub tag: String,        // TODO implement categories creation
    pub account_id: String, // TODO implement accounts creation
}

impl From<EntryPayload> for Entry {
    fn from(payload: EntryPayload) -> Self {
        Entry {
            entry_id: Uuid::new_v4(),
            invoice_id: payload.invoice_id,
            account_id: payload.account_id,
            entry_type: payload.entry_type,
            description: payload.description,
            value: payload.value,
            tag: payload.tag,
            due_date: payload.due_date,
            status: EntryStatus::Pending, // TODO: implement Default
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }
}
