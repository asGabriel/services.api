use std::sync::Arc;

pub mod entries;
pub mod invoices;

use crate::repositories::{entries::EntriesRepository, invoices::InvoicesRepository};

#[derive(Clone)]
pub struct Handler {
    invoices_repository: Arc<dyn InvoicesRepository + Send + Sync>,
    entries_repository: Arc<dyn EntriesRepository + Send + Sync>,
}

impl Handler {
    pub const fn new(
        invoice_repository: Arc<dyn InvoicesRepository + Send + Sync>,
        entries_repository: Arc<dyn EntriesRepository + Send + Sync>,
    ) -> Self {
        Self {
            invoices_repository: invoice_repository,
            entries_repository: entries_repository,
        }
    }
}
