
use crate::domains::{errors::Result, transactions::{report::PeriodFilter, Transaction}};

use super::Handler;

#[async_trait::async_trait]
pub trait ReportsHandler {
    async fn generate_report(&self, period: PeriodFilter) -> Result<Vec<Transaction>>;
}

impl Handler {
    pub async fn generate_report(&self, period: PeriodFilter) -> Result<Vec<Transaction>> {
        self.report_repository
            .list_transactions_by_period(period)
            .await
    }
}
