use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FinancialPlan {
    pub financial_plan_id: Uuid,
    pub title: Option<String>,
    pub month: MonthReference,
    pub year: i16,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

impl FinancialPlan {
    pub fn new_from_payload(payload: CreateFinancialPlan) -> Self {
        FinancialPlan {
            financial_plan_id: Uuid::new_v4(),
            title: Some(payload.title.unwrap_or_else(|| {
                format!("Financial plan - {:?}-{}", payload.month, payload.year)
            })),
            month: payload.month,
            year: payload.year,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateFinancialPlan {
    pub title: Option<String>,
    pub month: MonthReference,
    pub year: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "month_reference", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MonthReference {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
