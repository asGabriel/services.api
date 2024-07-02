use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::installment::PartialInstallment;

use super::{category::Category, movement_type::MovementType, status::TransactionStatus};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub transaction_id: Uuid,
    pub movement_type: MovementType,
    pub description: String,
    pub amount: BigDecimal,
    pub due_date: NaiveDate,
    pub category: Category,
    pub account_id: Uuid,
    pub status: TransactionStatus,
    pub installment_number: i16,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Transaction {
    /// FINISHED transaction is when the status equals to COMPLETED or CANCELED
    pub fn is_finished(&self) -> bool {
        match self.status {
            TransactionStatus::Completed | TransactionStatus::Canceled => true,
            _ => false,
        }
    }

    pub fn generate_installment_payload(&self) -> PartialInstallment {
        PartialInstallment {
            transaction_id: self.transaction_id,
            due_date: self.due_date,
            step: self.installment_number,
            amount: self.amount.normalized() / self.installment_number,
            status: TransactionStatus::Pending,
        }
    }
}
