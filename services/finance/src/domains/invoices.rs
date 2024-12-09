use bigdecimal::FromPrimitive;
use chrono::{DateTime, Month, Utc};
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
    pub is_parent: bool,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoicePayload {
    pub title: Option<String>,
    #[serde(deserialize_with = "validate_month")]
    pub month: i32,
    pub year: i16,
    pub is_parent: bool,
}

impl InvoicePayload {
    pub fn is_parent(&self) -> bool {
        self.is_parent
    }

    pub fn check_invalid_child_invoice(&self) -> bool {
        !self.is_parent && self.title.is_none()
    }
}

impl From<InvoicePayload> for Invoice {
    fn from(payload: InvoicePayload) -> Self {
        let month = Month::from_i32(payload.month).unwrap();

        Invoice {
            invoice_id: Uuid::new_v4(),
            title: payload.title.unwrap_or(format!(
                "Fatura de {} / {}",
                month.name(),
                payload.year
            )),
            month: payload.month,
            year: payload.year,
            is_parent: payload.is_parent,
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
