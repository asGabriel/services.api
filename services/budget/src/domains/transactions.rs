use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

use crate::update_fields;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub transaction_id: Uuid,
    pub movement_type: MovementType,
    pub description: String,
    pub value: BigDecimal,
    pub due_date: NaiveDate,
    pub category: Category,
    pub account_id: Uuid,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransaction {
    pub movement_type: MovementType,
    pub description: String,
    pub value: BigDecimal,
    pub due_date: NaiveDate,
    pub category: Category,
    pub account_id: Uuid,
    pub installments: i16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTransaction {
    pub movement_type: Option<MovementType>,
    pub description: Option<String>,
    pub value: Option<BigDecimal>,
    pub due_date: Option<NaiveDate>,
    pub category: Option<Category>,
    pub account_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "movement_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MovementType {
    Income,
    Expense,
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
#[sqlx(type_name = "category", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Category {
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

impl Default for Transaction {
    fn default() -> Self {
        Transaction {
            transaction_id: Uuid::new_v4(),
            account_id: Uuid::new_v4(),
            category: Category::Food,
            description: String::from("default value"),
            movement_type: MovementType::Expense,
            due_date: NaiveDate::default(),
            status: TransactionStatus::Pending,
            value: BigDecimal::default(),
            created_at: Utc::now(),
            deleted_at: None,
            updated_at: None,
        }
    }
}

impl Transaction {
    /// FINISHED transaction is when the status equals to COMPLETED or CANCELED
    pub fn is_finished(&self) -> bool {
        match self.status {
            TransactionStatus::Completed | TransactionStatus::Canceled => true,
            _ => false,
        }
    }

    /// prepare an transaction to be updated
    pub fn update(&mut self, data: UpdateTransaction) {
        update_fields!(
            self,
            data,
            movement_type,
            description,
            value,
            due_date,
            category,
            account_id
        );
        self.updated_at = Some(Utc::now());
    }

    pub fn from_payload(payload: CreateTransaction) -> Self {
        Transaction {
            transaction_id: Uuid::new_v4(),
            account_id: payload.account_id,
            description: payload.description,
            value: payload.value,
            category: payload.category,
            status: TransactionStatus::Pending,
            due_date: payload.due_date,
            movement_type: payload.movement_type,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }
}
