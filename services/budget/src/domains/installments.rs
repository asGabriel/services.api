use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::transactions::{Transaction, TransactionStatus};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Installment {
    pub installment_id: Uuid,
    pub transaction_id: Uuid,
    pub financial_plan_id: Uuid,
    pub installment_number: i16,
    pub total_installment: i16,
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
    pub financial_plan_id: Uuid,
    pub due_date: NaiveDate,
    pub value: BigDecimal,
    pub status: TransactionStatus,
    pub params: InstallmentParams,
}

#[derive(Debug, Deserialize)]
pub struct InstallmentParams {
    pub installment_number: i16,
    pub total_installment: i16,
}

#[derive(Debug, Deserialize)]
pub struct CreateInstallment {
    pub transaction_id: Uuid,
    pub financial_plan_id: Uuid,
    pub installment_number: i16,
    pub due_date: NaiveDate,
    pub value: BigDecimal,
    pub status: TransactionStatus,
}

impl InstallmentParams {
    pub fn new(number: i16, total: i16) -> Self {
        InstallmentParams {
            installment_number: number,
            total_installment: total,
        }
    }
}

impl PartialInstallment {
    pub fn from_payload(payload: &Transaction, params: &InstallmentParams, financial_plan_id: Uuid) -> Self {
        PartialInstallment {
            transaction_id: payload.transaction_id,
            financial_plan_id: financial_plan_id,
            due_date: payload.due_date,
            status: payload.status,
            value: payload.value.normalized(),
            params: InstallmentParams::new(params.installment_number, params.total_installment),
        }
    }
}

impl Installment {
    pub fn is_finished(&self) -> bool {
        match self.status {
            TransactionStatus::Completed | TransactionStatus::Canceled => true,
            _ => false,
        }
    }
}
