use std::sync::Arc;

pub mod invoices;

use crate::repositories::invoices::InvoicesRepository;

#[derive(Clone)]
pub struct Handler {
    invoice_repository: Arc<dyn InvoicesRepository + Send + Sync>,
}

impl Handler {
    pub const fn new(invoice_repository: Arc<dyn InvoicesRepository + Send + Sync>) -> Self {
        Self {
            invoice_repository: invoice_repository
        }
    }
}