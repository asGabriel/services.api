use bigdecimal::BigDecimal;
use serde::Deserialize;

use super::MonthReference;

#[derive(Debug, Deserialize)]
pub struct PeriodFilter {
    pub month: i16,
    pub year: BigDecimal,
}

impl PeriodFilter {
    pub fn transform_month(&self) -> MonthReference {
        match self.month {
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
            _ => MonthReference::January,
        }
    }
}
