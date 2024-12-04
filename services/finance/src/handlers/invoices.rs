use app_shared::finance::invoices::Invoice;
use http_problems::{Error, Result};
use uuid::Uuid;

use crate::domains::invoices::InvoicePayload;

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
        let exists_invoice = self
            .invoices_repository
            .get_invoice_by_reference(&payload)
            .await?;

        if exists_invoice.is_some() {
            return Err(Error::ConflictError(format!(
                "Already exists an invoice of {}-{}",
                payload.year, payload.month
            )));
        }

        let invoice = payload.into();

        self.invoices_repository.create_invoice(invoice).await
    }
}
