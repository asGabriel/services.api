use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub transaction_id: Uuid,
    pub movement_type: TransactionType,
    pub description: Option<String>,
    pub amount: f64,
    pub due_date: Option<NaiveDate>,
    pub category: Option<TransactionCategory>,
    pub account_id: Uuid,
    pub recurring: Option<bool>,
    pub recurrence_frequency: Option<TransactionRecurrency>,
    pub recurrence_duration_months: Option<i32>,
    pub note: Option<String>,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionDto {
    pub movement_type: TransactionType,
    pub description: Option<String>,
    pub amount: f64,
    pub due_date: Option<NaiveDate>,
    pub category: TransactionCategory,
    pub account_id: Uuid,
    pub recurring: Option<bool>,
    pub recurrence_frequency: TransactionRecurrency,
    pub recurrence_duration_months: Option<i32>,
    pub note: Option<String>,
    pub status: TransactionStatus,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "movement_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Income,
    Expense,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "category", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionCategory {
    Food,
    Home,
    Education,
    PersonalExpense,
    Sporadic,
    Entertainment,
    Transport,
    Healthy,
    Salary,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    Pending,
    Canceled,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(
    type_name = "recurrence_frequency",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum TransactionRecurrency {
    Weekly,
    Monthly,
    Quarterly,
    SemiAnnually,
    Annually,
}
