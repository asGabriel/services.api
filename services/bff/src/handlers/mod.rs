use std::sync::Arc;

use finance_client::invoices::InvoicesGateway;

pub mod invoices;

#[derive(Clone)]
pub struct Handler {
    invoices_gateway: Arc<dyn InvoicesGateway + Send + Sync>,
}

impl Handler {
    pub const fn new(invoices_gateway: Arc<dyn InvoicesGateway + Send + Sync>) -> Self {
        Self {
            invoices_gateway: invoices_gateway,
        }
    }
}
