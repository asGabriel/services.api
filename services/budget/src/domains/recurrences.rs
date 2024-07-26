use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

use crate::update_fields;

use super::transactions::{Category, MovementType};

#[derive(Debug, Serialize)]
pub struct Recurrence {
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

#[derive(Debug, Serialize)]
pub struct RecurrenceLink {
    pub transaction_id: Uuid,
    pub due_date: NaiveDate
}

#[derive(Debug, Deserialize, Serialize, Type)]
#[sqlx(type_name = "frequency")]
pub enum Frequency {
    MONTHLY,
    WEEKLY,
    ANNUALLY,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecurrence {
    pub account_id: Uuid,
    pub title: String,
    pub frequency: Frequency,
    pub category: Category,
    #[serde(default)]
    pub is_active: bool,
    pub start_date: NaiveDate,
    pub value: BigDecimal,
    pub movement_type: MovementType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRecurrence {
    pub account_id: Option<Uuid>,
    pub title: Option<String>,
    pub frequency: Option<Frequency>,
    pub category: Option<Category>,
    pub is_active: Option<bool>,
    pub start_date: Option<NaiveDate>,
    pub value: Option<BigDecimal>,
    pub movement_type: Option<MovementType>,
}

impl Recurrence {
    pub fn new_from_payload(payload: CreateRecurrence) -> Self {
        Recurrence {
            recurrence_id: Uuid::new_v4(),
            account_id: payload.account_id,
            title: payload.title,
            frequency: payload.frequency,
            category: payload.category,
            is_active: payload.is_active,
            start_date: payload.start_date,
            value: payload.value,
            movement_type: payload.movement_type,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }

    /// prepare a recurrence to be updated
    pub fn update(&mut self, data: UpdateRecurrence) {
        update_fields!(
            self,
            data,
            account_id,
            title,
            frequency,
            category,
            is_active,
            start_date,
            value,
            movement_type
        );
        self.updated_at = Some(Utc::now());
    }
}
