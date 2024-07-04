use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use finance::{frequency::Frequency, status::TransactionStatus};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::transactions::MonthReference;

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
#[serde(rename_all = "camelCase")]
pub struct InstallmentParams {
    pub month_reference: MonthReference,
    pub year_reference: i16,
    pub recurrence_frequency: Frequency,
}

#[derive(Debug, Deserialize)]
pub struct CreateInstallment {
    pub transaction_id: Uuid,
    pub installment_number: i16,
    pub due_date: NaiveDate,
    pub value: BigDecimal,
    pub status: TransactionStatus,
}
