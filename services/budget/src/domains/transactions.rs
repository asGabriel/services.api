use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use finance::{
    category::Category, frequency::Frequency, movement_type::MovementType,
    status::TransactionStatus,
};
use serde::Deserialize;
use uuid::Uuid;

pub mod report;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransaction {
    pub movement_type: MovementType,
    pub description: String,
    pub value: BigDecimal,
    pub due_date: NaiveDate,
    pub category: Category,
    pub account_id: Uuid,
    pub status: TransactionStatus,
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
    pub recurring: Option<bool>,
    pub recurrence_frequency: Option<Frequency>,
    pub recurrence_duration_months: Option<i32>,
    pub note: Option<String>,
    pub status: Option<TransactionStatus>,
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
