use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

use super::installments::CreateInstallment;

pub mod report;

#[derive(Serialize, Debug, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransaction {
    pub movement_type: TransactionType,
    pub description: String,
    pub amount: BigDecimal,
    pub due_date: Option<NaiveDate>,
    pub category: TransactionCategory,
    pub account_id: Uuid,
    pub status: TransactionStatus,
    pub installment_number: Option<i16>,
    pub month_reference: MonthReference,
    pub year_reference: i16
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTransaction {
    pub movement_type: Option<TransactionType>,
    pub description: Option<String>,
    pub amount: Option<BigDecimal>,
    pub due_date: Option<NaiveDate>,
    pub category: Option<TransactionCategory>,
    pub account_id: Option<Uuid>,
    pub recurring: Option<bool>,
    pub recurrence_frequency: Option<TransactionRecurrency>,
    pub recurrence_duration_months: Option<i32>,
    pub note: Option<String>,
    pub status: Option<TransactionStatus>,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "movement_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Income,
    Expense,
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

#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    Pending,
    Canceled,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(
    type_name = "recurrence_frequency",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum TransactionRecurrency {
    SingleOccurrence,
    Weekly,
    Monthly,
    Quarterly,
    SemiAnnually,
    Annually,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "month_reference", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MonthReference {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Transaction {
    /// FINISHED transaction is when the status equals to COMPLETED or CANCELED
    pub fn is_finished(&self) -> bool {
        match self.status {
            TransactionStatus::Completed | TransactionStatus::Canceled => true,
            _ => false,
        }
    }

    pub fn generate_installment_payload(&self) -> CreateInstallment {
        CreateInstallment {
            transaction_id: self.transaction_id,
            due_date: self.due_date,
            step: self.installment_number,
            amount: self.amount.normalized() / self.installment_number,
            status: TransactionStatus::Pending
        }
    }
}
