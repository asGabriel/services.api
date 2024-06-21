use std::num::NonZeroU16;

use finance::status::TransactionStatus;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransactionFilterParams {
    pub status: TransactionStatus,
    pub pagination: Option<PaginationParameters>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParameters {
    #[serde(default = "default_page")]
    pub page: NonZeroU16,
    #[serde(default = "default_size")]
    pub size: NonZeroU16,
}

fn default_page() -> NonZeroU16 {
    NonZeroU16::new(1).unwrap()
}

fn default_size() -> NonZeroU16 {
    NonZeroU16::new(30).unwrap()
}
