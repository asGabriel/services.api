use bigdecimal::BigDecimal;
use chrono::{DateTime, Local, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Settlement {
    pub settlement_id: Uuid,
    pub transaction_id: Uuid,
    pub installment_id: Option<Uuid>,
    pub paid_date: NaiveDate,
    pub paid_value: BigDecimal,
    pub discount: Option<BigDecimal>,
    pub fees: Option<BigDecimal>,
    pub attachment: Option<Vec<u8>>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSettlement {
    pub paid_date: NaiveDate,
    pub paid_value: BigDecimal,
    pub discount: Option<BigDecimal>,
    pub fees: Option<BigDecimal>,
    pub attachment: Option<Vec<u8>>,
}

#[derive(Debug, Deserialize)]
pub struct SettlementParams {
    pub transaction_id: Uuid,
    pub installment_id: Option<Uuid>,
}

impl Settlement {
    pub fn new_from_payload(payload: CreateSettlement, params: SettlementParams) -> Self {
        Settlement {
            settlement_id: Uuid::new_v4(),
            transaction_id: params.transaction_id,
            installment_id: params.installment_id,
            paid_value: payload.paid_value,
            paid_date: payload.paid_date,
            discount: payload.discount,
            fees: payload.fees,
            attachment: payload.attachment,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }
}
