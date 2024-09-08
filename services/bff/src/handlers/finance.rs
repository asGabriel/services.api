use finance_domains::entries::EntryStatus;
use http_problems::errors::Result;

use crate::domains::operations::{InvoiceWithEntriesDetails, OperationsPage};

use super::Handler;

impl Handler {
    pub async fn get_operations(&self) -> Result<OperationsPage> {
        let invoices = self.finance_gateway.list_invoices().await?;
        let entries = self.finance_gateway.list_entries().await?;

        let operations = invoices
            .iter()
            .map(|invoice| {
                let related_entries = entries
                    .iter()
                    .filter(|e| e.status == EntryStatus::Completed)
                    .collect::<Vec<_>>();

                return InvoiceWithEntriesDetails::build(invoice, related_entries);
            })
            .collect::<Vec<InvoiceWithEntriesDetails>>();

        Ok(OperationsPage {
            total: invoices.len(),
            operations: operations,
        })
    }
}
