use bigdecimal::FromPrimitive;
use chrono::{DateTime, Month, Utc};
use serde::{de::{self, Unexpected}, Deserialize, Deserializer, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub invoice_id: Uuid,
    pub title: Option<String>,
    pub month: i32,
    pub year: i16,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoicePayload {
    #[serde(deserialize_with = "validate_month")]
    pub month: i32,
    pub year: i16
}

impl From<InvoicePayload> for Invoice {
    fn from(payload: InvoicePayload) -> Self {
        let month = Month::from_i32(payload.month).unwrap();

        Invoice {
            invoice_id: Uuid::new_v4(),
            title: Some(format!("Fatura de {} / {}", month.name(), payload.year)),
            month: payload.month,
            year: payload.year,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None
        }
    }
}

fn validate_month<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let month = i32::deserialize(deserializer)?;
    if (1..=12).contains(&month) {
        Ok(month)
    } else {
        Err(de::Error::invalid_value(Unexpected::Signed(month as i64), &"a number between 1 and 12"))
    }
}
