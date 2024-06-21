use bigdecimal::BigDecimal;
use chrono::{DateTime, Months, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{month::MonthReference, status::TransactionStatus};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Installment {
    pub installment_id: Uuid,
    pub transaction_id: Uuid,
    pub step: i16,
    pub due_date: NaiveDate,
    pub amount: BigDecimal,
    pub status: TransactionStatus,
    pub payment_date: Option<NaiveDate>,
    pub month_reference: MonthReference,
    pub year_reference: i16,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct PartialInstallment {
    pub transaction_id: Uuid,
    pub step: i16,
    pub due_date: NaiveDate,
    pub amount: BigDecimal,
    pub status: TransactionStatus,
}

impl PartialInstallment {
    pub fn next_due_date_by_frequency(&mut self) {
        self.due_date = self.due_date.checked_add_months(Months::new(1)).unwrap()
    }
}
