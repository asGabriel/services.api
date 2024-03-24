use ::serde::Serialize;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkNote {
    pub work_note_id: Uuid,
    pub category: String,
    pub work_date: NaiveDate,
    pub work_hours: f64,
    pub observation: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkNote {
    pub category: String,
    pub work_date: NaiveDate,
    pub work_hours: f64,
    pub observation: Option<String>,
}
