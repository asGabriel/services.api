use finance_domains::Invoice;
use uuid::Uuid;

use crate::domains::{
    errors::{Error, Result},
    invoices::InvoicePayload,
};

use super::Handler;

impl Handler {
    pub async fn list_invoices(&self) -> Result<Vec<Invoice>> {
        self.invoices_repository.list_invoices().await
    }

    pub async fn get_invoice_by_id(&self, invoice_id: Uuid) -> Result<Invoice> {
        self.invoices_repository
            .get_invoice_by_id(invoice_id)
            .await?
            .ok_or(Error::InvoiceNotFound(invoice_id))
    }

    pub async fn create_invoice(&self, payload: InvoicePayload) -> Result<Invoice> {
        let invoice = payload.into();

        self.invoices_repository.create_invoice(invoice).await
    }
}
