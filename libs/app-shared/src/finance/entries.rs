use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub entry_id: Uuid,
    pub invoice_id: Uuid,
    pub entry_type: EntryType,
    pub description: String,
    pub value: BigDecimal,
    pub due_date: NaiveDate,
    // TODO implement categories creation
    pub tag: String,
    pub account_id: Uuid,
    pub status: EntryStatus,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "entry_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntryStatus {
    Pending,
    Canceled,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "entry_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntryType {
    Revenue,
    Expense,
}
