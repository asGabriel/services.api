use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "category", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Category {
    Food,
    Home,
    Education,
    Entertainment,
    Transport,
    Healthy,
    Salary,
    Utilities,
    Insurance,
    Savings,
    DebtPayments,
    ChildCare,
    Gifts,
    Subscriptions,
    Travel,
    Clothing,
    Maintenance,
}
