use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_id: Uuid,
    pub bank_name: Bank,
    pub owner: String,
    pub account_type: AccountType,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccount {
    pub bank_name: Bank,
    pub owner: String,
    pub account_type: AccountType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAccount {
    pub bank_name: Option<Bank>,
    pub owner: Option<String>,
    pub account_type: Option<AccountType>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
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

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "account_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    Debit,
    Credit,
    Hybrid,
}
