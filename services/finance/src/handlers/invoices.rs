use chrono::{Datelike, Utc};
use http_problems::{Error, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domains::{
    invoice_relations::InvoiceRelations,
    invoices::{Invoice, InvoicePayload, InvoiceUpdatePayload},
};

use super::Handler;

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceReferenceParams {
    pub year: i32,
    pub month: u32,
}

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
        let now = Utc::now();
        let (year, month) = (now.year(), now.month());

        let current_invoice = self
            .invoices_repository
            .get_main_invoice_by_reference(InvoiceReferenceParams { year, month })
            .await?;

        if current_invoice.is_some() {
            // TODO: implementar log
            Ok(())
        } else {
            let _invoice = self
                .invoices_repository
                .create_invoice(Invoice::default())
                .await?;

            Ok(())
        }
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
            let (year, month) = (now.year(), now.month());

            self.invoices_repository
                .get_main_invoice_by_reference(InvoiceReferenceParams { year, month })
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
