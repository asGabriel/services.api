use bigdecimal::BigDecimal;
use chrono::{DateTime, Months, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::status::TransactionStatus;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Installment {
    pub installment_id: Uuid,
    pub transaction_id: Uuid,
    pub installment_number: i16,
    pub due_date: NaiveDate,
    pub value: BigDecimal,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct PartialInstallment {
    pub transaction_id: Uuid,
    pub installment_number: i16,
    pub due_date: NaiveDate,
    pub value: BigDecimal,
    pub status: TransactionStatus,
}

impl PartialInstallment {
    pub fn next_due_date_by_frequency(&mut self) {
        self.due_date = self.due_date.checked_add_months(Months::new(1)).unwrap()
    }
}
