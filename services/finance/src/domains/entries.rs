use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub entry_id: Uuid,
    pub invoice_id: Uuid,
    pub entry_type: EntryType,
    pub description: String,
    pub value: BigDecimal,
    pub due_date: NaiveDate,
    pub tag: String,        // TODO implement categories creation
    pub account_id: String, // TODO implement accounts creation
    pub status: EntryStatus,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

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

#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "entry_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntryType {
    Revenue,
    Expense,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "entry_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntryStatus {
    Pending,
    Canceled,
    Completed,
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
            status: EntryStatus::Pending,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }
}
