use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use finance::{
    category::TransactionCategory, movement_type::MovementType, status::TransactionStatus,
};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

pub mod report;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransaction {
    pub movement_type: MovementType,
    pub description: String,
    pub amount: BigDecimal,
    pub due_date: Option<NaiveDate>,
    pub category: TransactionCategory,
    pub account_id: Uuid,
    pub status: TransactionStatus,
    pub installment_number: Option<i16>,
    pub month_reference: MonthReference,
    pub year_reference: i16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTransaction {
    pub movement_type: Option<MovementType>,
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
