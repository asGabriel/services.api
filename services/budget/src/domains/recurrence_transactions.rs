use bigdecimal::BigDecimal;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct RecurrenceTransaction {
    pub recurrence_transaction_id: Uuid,
    pub account_id: Uuid,
    pub transaction_id: Uuid,
    pub description: String,
    pub amount: BigDecimal,
}