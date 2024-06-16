use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub transaction_id: Uuid,
    pub movement_type: TransactionType,
    pub description: String,
    pub amount: BigDecimal,
    pub due_date: NaiveDate,
    pub category: TransactionCategory,
    pub account_id: Uuid,
    pub status: TransactionStatus,
    pub installment_number: i16,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    Pending,
    Canceled,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "category", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionCategory {
    Food,
    Home,
    Education,
    Entertainment,
    Transport,
    Healthy,
    Salary,
    Utilities,
    Insurance,
    Savings,
    DebtPayments,
    ChildCare,
    Gifts,
    Subscriptions,
    Travel,
    Clothing,
    Maintenance,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "movement_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Income,
    Expense,
}
