use chrono::{Datelike, Months, NaiveDate, Utc};
use http_problems::{Error, Result};
use uuid::Uuid;

use crate::domains::{
    invoice_relations::InvoiceRelations,
    invoices::{Invoice, InvoicePayload, InvoiceUpdatePayload},
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
        let current_invoice = if let Some(id) = payload.main_invoice {
            self.get_invoice_by_id(id).await?
        } else {
            let now = Utc::now();
            let current = (now.year(), now.month() as i32);

            self.invoices_repository
                .get_invoice_by_referece(current)
                .await?
                .unwrap()
        };

        let new_invoice = self
            .invoices_repository
            .create_invoice(payload.into())
            .await?;

        self.invoice_relations_repository
            .create_relations(InvoiceRelations {
                parent_invoice_id: current_invoice.invoice_id,
                child_invoice_id: new_invoice.invoice_id,
            })
            .await?;

        Ok(new_invoice)
    }
}
