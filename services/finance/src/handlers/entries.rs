use http_problems::{Error, Result};
use uuid::Uuid;

use crate::domains::{
    entries::{Entry, EntryPayload},
    views::views::{InvoiceDetails, InvoiceDetailsView},
};

use super::Handler;

impl Handler {
    pub async fn delete_entry_by_id(&self, entry_id: Uuid) -> Result<()> {
        self.entries_repository.delete_entry_by_id(entry_id).await?;

        Ok(())
    }

    pub async fn list_entries(&self) -> Result<Vec<Entry>> {
        self.entries_repository.list_entries().await
    }

    pub async fn get_entry_by_id(&self, entry_id: Uuid) -> Result<Entry> {
        self.entries_repository
            .get_entry_by_id(entry_id)
            .await?
            .ok_or(Error::NotFoundError(format!(
                "Entry id {entry_id} not found.",
            )))
    }

    pub async fn create_entry(&self, payload: EntryPayload) -> Result<Entry> {
        let tags = self.insert_many_tags(payload.clone().tag).await?;

        let entry = self.entries_repository.create_entry(payload.into()).await?;
        self.tags_repository
            .insert_many_entries_tags_relation(tags, entry.entry_id)
            .await?;

        Ok(entry)
    }

    pub async fn list_entries_by_invoice_id(&self, invoice_id: Uuid) -> Result<InvoiceDetailsView> {
        let mut invoice_details: Vec<InvoiceDetails> = Vec::new();

        let invoice = self.get_invoice_by_id(invoice_id).await?;

        let entries = self
            .entries_repository
            .get_entries_by_invoice_id(invoice_id)
            .await?;

        let main_invoice = InvoiceDetails {
            invoice: invoice.clone(),
            entries
        };

        let sub_invoices = self
            .invoice_relations_repository
            .list_related_invoices(&invoice)
            .await?;
        for sub_invoice in sub_invoices {
            let invoice = self
                .get_invoice_by_id(sub_invoice.child_invoice_id)
                .await?;
            let entries = self
                .entries_repository
                .get_entries_by_invoice_id(invoice.invoice_id)
                .await?;

            invoice_details.push(InvoiceDetails { invoice, entries });
        }

        Ok(InvoiceDetailsView {
            main_invoice,
            sub_invoices: invoice_details,
        })
    }
}
