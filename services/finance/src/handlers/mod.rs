use std::sync::Arc;

pub mod accounts;
pub mod entries;
pub mod invoices;

use crate::repositories::{
    accounts::AccountRepository, entries::EntriesRepository, invoices::InvoicesRepository,
};

#[derive(Clone)]
pub struct Handler {
    invoices_repository: Arc<dyn InvoicesRepository + Send + Sync>,
    entries_repository: Arc<dyn EntriesRepository + Send + Sync>,
    accounts_repository: Arc<dyn AccountRepository + Send + Sync>,
}

impl Handler {
    pub const fn new(
        invoices_repository: Arc<dyn InvoicesRepository + Send + Sync>,
        entries_repository: Arc<dyn EntriesRepository + Send + Sync>,
        accounts_repository: Arc<dyn AccountRepository + Send + Sync>,
    ) -> Self {
        Self {
            invoices_repository,
            entries_repository,
            accounts_repository,
        }
    }
}
