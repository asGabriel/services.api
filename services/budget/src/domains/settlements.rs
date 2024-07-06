use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
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
    pub deleted_at: Option<DateTime<Utc>>
}

#[derive(Debug, Deserialize)]
pub struct CreateSettlement {
    pub transaction_id: Uuid,
    pub installment_id: Option<Uuid>,
    pub paid_date: NaiveDate,
    pub paid_value: BigDecimal,
    pub discount: Option<BigDecimal>,
    pub fees: Option<BigDecimal>,
    pub attachment: Option<Vec<u8>>,
}