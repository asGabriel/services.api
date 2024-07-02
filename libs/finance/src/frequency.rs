use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(
    type_name = "recurrence_frequency",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum Frequency {
    SingleOccurrence,
    Weekly,
    Monthly,
    Quarterly,
    SemiAnnually,
    Annually,
}
