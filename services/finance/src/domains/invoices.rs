use bigdecimal::FromPrimitive;
use chrono::{DateTime, Datelike, Month, Utc};
use http_problems::{Error, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub invoice_id: Uuid,
    pub title: String,
    pub month: i32,
    pub year: i32,
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
            year: year as i32,
            is_main: true,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }
}

impl Invoice {
    pub fn new(year: i32, month: i32, main: Option<bool>) -> Self {
        Self {
            invoice_id: Uuid::new_v4(),
            title: format!("Fatura de {}/{}", month, year),
            month,
            year,
            is_main: main.unwrap_or(false),
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }

    pub fn new_from_payload(&self, title: String) -> Self {
        Self {
            invoice_id: Uuid::new_v4(),
            title,
            month: self.month,
            year: self.year,
            is_main: false,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }

    pub fn update(&mut self, payload: InvoiceUpdatePayload) {
        if let Some(title) = payload.title {
            self.title = title;
        }
        if let Some(month) = payload.month {
            self.month = month;
        }
        if let Some(year) = payload.year {
            self.year = year;
        }

        self.updated_at = Some(Utc::now());
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoicePayload {
    pub title: String,
    pub main_invoice: Uuid,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceUpdatePayload {
    pub title: Option<String>,
    pub month: Option<i32>,
    pub year: Option<i32>,
}

impl InvoiceUpdatePayload {
    pub fn sanitize(&self) -> Result<()> {
        if let Some(month) = self.month {
            is_valid_month(month)?;
        }

        Ok(())
    }
}

fn is_valid_month(month: i32) -> Result<i32> {
    if (1..=12).contains(&month) {
        Ok(month)
    } else {
        Err(Error::InvalidEntityError(
            "Month value must be a number between 1 and 12".to_string(),
        ))
    }
}
