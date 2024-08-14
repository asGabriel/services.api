use chrono::{DateTime, Datelike, NaiveDate, Utc};
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

impl CreateFinancialPlan {
    pub fn new(due_date: NaiveDate) -> Self {
        CreateFinancialPlan {
            title: Some(format!(
                "Financial plan - {:?}-{}",
                due_date.month(),
                due_date.year()
            )),
            month: MonthReference::from(due_date.month() as i16),
            year: due_date.year() as i16,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "month_reference", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MonthReference {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl From<i16> for MonthReference {
    fn from(value: i16) -> Self {
        match value {
            1 => MonthReference::January,
            2 => MonthReference::February,
            3 => MonthReference::March,
            4 => MonthReference::April,
            5 => MonthReference::May,
            6 => MonthReference::June,
            7 => MonthReference::July,
            8 => MonthReference::August,
            9 => MonthReference::September,
            10 => MonthReference::October,
            11 => MonthReference::November,
            12 => MonthReference::December,
            _ => panic!("Invalid value for MonthReference: {}", value),
        }
    }
}
