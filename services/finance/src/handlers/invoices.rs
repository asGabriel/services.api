use uuid::Uuid;

use crate::domains::{
    errors::{Error, Result},
    invoices::{Invoice, InvoicePayload},
};

use super::Handler;

impl Handler {
    pub async fn list_invoices(&self) -> Result<Vec<Invoice>> {
        self.invoice_repository.list_invoices().await
    }

    pub async fn get_invoice_by_id(&self, invoice_id: Uuid) -> Result<Invoice> {
        self.invoice_repository
            .get_invoice_by_id(invoice_id)
            .await?
            .ok_or(Error::InvoiceNotFound(invoice_id))
    }

    pub async fn create_invoice(&self, payload: InvoicePayload) -> Result<Invoice> {
        let invoice = Invoice::new_from_payload(payload);

        self.invoice_repository.create_invoice(invoice).await
    }
}
