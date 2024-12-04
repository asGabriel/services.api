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
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccountPayload {
    pub bank_name: Bank,
    pub owner: String,
}

impl From<CreateAccountPayload> for Account {
    fn from(value: CreateAccountPayload) -> Self {
        Account {
            account_id: Uuid::new_v4(),
            bank_name: value.bank_name,
            owner: value.owner,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }
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
    Swile,
}
