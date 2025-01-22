use bigdecimal::FromPrimitive;
use chrono::{DateTime, Datelike, Month, Utc};
use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer, Serialize,
};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub invoice_id: Uuid,
    pub title: String,
    pub month: i32,
    pub year: i16,
    pub is_main: bool,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Default for Invoice {
    fn default() -> Self {
        let now = Utc::now();
        let (year, month) = (now.year(), now.month());

        let month_name = Month::from_u32(month).unwrap();

        Self {
            invoice_id: Uuid::new_v4(),
            title: format!("Fatura de {} / {}", month_name.name(), year),
            month: month as i32,
            year: year as i16,
            is_main: true,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoicePayload {
    pub title: String,
    #[serde(deserialize_with = "validate_month")]
    pub month: i32,
    pub year: i16,
    pub main_invoice: Option<Uuid>
}

impl From<InvoicePayload> for Invoice {
    fn from(payload: InvoicePayload) -> Self {
        Invoice {
            invoice_id: Uuid::new_v4(),
            title: payload.title,
            month: payload.month,
            year: payload.year,
            is_main: false,
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
