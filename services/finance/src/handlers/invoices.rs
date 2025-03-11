use chrono::{Datelike, Months, NaiveDate, Utc};
use http_problems::{Error, Result};
use uuid::Uuid;

use crate::domains::{
    invoice_relations::InvoiceRelations,
    invoices::{Invoice, InvoicePayload, InvoiceUpdatePayload}, views::views::{InvoiceDetails, InvoiceDetailsView},
};

use super::Handler;

impl Handler {
    pub async fn update_invoice_by_id(
        &self,
        invoice_id: Uuid,
        payload: InvoiceUpdatePayload,
    ) -> Result<Invoice> {
        payload.sanitize()?;
        let mut invoice = self.get_invoice_by_id(invoice_id).await?;
        invoice.update(payload);

        self.invoices_repository
            .update_invoice_by_id(invoice)
            .await?
            .ok_or(Error::NotFoundError(format!(
                "Could not found invoice id {invoice_id} when update."
            )))
    }

    pub async fn create_monthly_main_invoice(&self) -> Result<()> {
        let mut current = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let now = Utc::now();
        let mut references = Vec::new();

        while current <= now.date_naive() {
            let month = current.month();
            let year = current.year_ce().1;

            references.push((year as i32, month as i32));
            current = current.checked_add_months(Months::new(1)).unwrap();
        }

        let current_invoices = self
            .invoices_repository
            .list_main_invoices_by_references(references.as_slice())
            .await?;

        // TODO: implementar log
        for reference in references {
            if current_invoices.contains_key(&reference) {
                continue;
            }

            let _invoice = self
                .invoices_repository
                .create_invoice(Invoice::new(reference.0, reference.1, Some(true)))
                .await?;
        }

        Ok(())
    }

    pub async fn list_invoices(&self) -> Result<Vec<Invoice>> {
        self.invoices_repository.list_invoices().await
    }

    pub async fn get_invoice_by_id(&self, invoice_id: Uuid) -> Result<Invoice> {
        self.invoices_repository
            .get_invoice_by_id(invoice_id)
            .await?
            .ok_or(Error::NotFoundError(format!(
                "Invoice id {invoice_id} not found.",
            )))
    }

    pub async fn create_invoice(&self, payload: InvoicePayload) -> Result<Invoice> {
        let current_invoice = self.get_invoice_by_id(payload.main_invoice).await?;
        let new_invoice = current_invoice.new_from_payload(payload.title);

        let new_invoice = self
            .invoices_repository
            .create_invoice(new_invoice)
            .await?;

        self.invoice_relations_repository
            .create_relations(InvoiceRelations {
                parent_invoice_id: current_invoice.invoice_id,
                child_invoice_id: new_invoice.invoice_id,
            })
            .await?;

        Ok(new_invoice)
    }

    pub async fn get_invoice_and_subinvoices_entries(&self, invoice_id: Uuid) -> Result<InvoiceDetailsView> {
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
