use crate::domains::{errors::Result, invoices::Invoice};

use super::Handler;

impl Handler {
    pub async fn list_invoices(&self) -> Result<Vec<Invoice>> {
        self.invoice_repository.list_invoices().await
    }
}