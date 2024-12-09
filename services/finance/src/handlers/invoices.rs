use http_problems::{Error, Result};
use uuid::Uuid;

use crate::domains::invoices::{Invoice, InvoicePayload};

use super::Handler;

impl Handler {
    pub async fn list_invoices(&self) -> Result<Vec<Invoice>> {
        self.invoices_repository.list_invoices().await
    }

    pub async fn get_invoice_by_id(&self, invoice_id: Uuid) -> Result<Invoice> {
        self.invoices_repository
            .get_invoice_by_id(invoice_id)
            .await?
            .ok_or(Error::NotFoundError(format!(
                "Entry id {invoice_id} not found.",
            )))
    }

    pub async fn create_invoice(&self, payload: InvoicePayload) -> Result<Invoice> {
        if payload.check_invalid_child_invoice() {
            return Err(Error::BadRequestError(
                "Child invoice must have a title".to_string(),
            ));
        }

        let exists_invoice = self
            .invoices_repository
            .get_invoice_by_reference(&payload)
            .await?;

        if exists_invoice.is_some() && payload.is_parent() {
            return Err(Error::ConflictError(format!(
                "Already exists an invoice of {}-{}",
                payload.year, payload.month
            )));
        }

        let invoice = payload.into();

        self.invoices_repository.create_invoice(invoice).await
    }
}
