use crate::domains::{
    errors::Result,
    transactions::report::{PeriodFilter, Report},
};

use super::Handler;

impl Handler {
    pub async fn generate_report(&self, period: PeriodFilter) -> Result<Report> {
        let transactions = self
            .report_repository
            .list_transactions_by_period(&period)
            .await?;

        let report = Report {
            month: period.transform_month(),
            year: period.year,
            transactions: transactions,
        };

        Ok(report)
    }
}
