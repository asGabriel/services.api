use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::Type;

use super::transactions::{Category, MovementType};

#[derive(Debug, Serialize)]
pub struct Recurrences {
    pub recurrence_id: Uuid,
    pub account_id: Uuid,
    pub title: String,
    pub frequency: Frequency,
    pub category: Category,
    pub is_active: bool,
    pub start_date: NaiveDate,
    pub value: BigDecimal,
    pub movement_type: MovementType,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Frequency {
    MONTHLY,
    WEEKLY,
    ANNUALLY
}
