use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Entries {
    pub transaction_id: Uuid,
    pub financial_plan_id: Uuid,
    pub entry_type: EntryType,
    pub description: String,
    pub value: BigDecimal,
    pub due_date: NaiveDate,
    pub category: String, // TODO implement categories creation
    pub account_id: String, // TODO implement accounts creation
    pub status: EntryStatus,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
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
