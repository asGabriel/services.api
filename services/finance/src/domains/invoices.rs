use bigdecimal::FromPrimitive;
use chrono::{Month, Utc};
use app_shared::finance::invoices::Invoice;
use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoicePayload {
    #[serde(deserialize_with = "validate_month")]
    pub month: i32,
    pub year: i16,
}

impl From<InvoicePayload> for Invoice {
    fn from(payload: InvoicePayload) -> Self {
        let month = Month::from_i32(payload.month).unwrap();

        Invoice {
            invoice_id: Uuid::new_v4(),
            title: format!("Fatura de {} / {}", month.name(), payload.year),
            month: payload.month,
            year: payload.year,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
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
        Err(de::Error::invalid_value(
            Unexpected::Signed(month as i64),
            &"a number between 1 and 12",
        ))
    }
}
