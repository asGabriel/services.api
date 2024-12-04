use app_shared::finance::entries::{Entry, EntryStatus};
use http_problems::Result;
use uuid::Uuid;

use crate::domains::operations::{InvoiceDetailsPage, InvoiceWithEntriesDetails, OperationsPage};

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

    pub async fn get_entries_by_invoice_id(&self, invoice_id: Uuid) -> Result<InvoiceDetailsPage> {
        let invoice = self.finance_gateway.get_invoice_by_id(invoice_id).await?;

        let entries = self
            .finance_gateway
            .list_entries_by_invoice_id(invoice_id)
            .await?;

        let entries_ref = entries.iter().collect::<Vec<&Entry>>();
        let details = InvoiceWithEntriesDetails::build(&invoice, entries_ref);

        let details_page = InvoiceDetailsPage {
            invoice: details,
            entries: entries,
        };

        Ok(details_page)
    }
}
