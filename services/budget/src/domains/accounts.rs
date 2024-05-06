use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::Type;
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_id: Uuid,
    pub name: String,
    pub bank_name: Bank,
    pub owner: String,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "bank_name", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Bank {
    Nubank,
    Inter,
    Santander,
    Itau,
    Bradesco,
    BancoDoBrasil,
}
