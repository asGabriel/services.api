use std::sync::Arc;

pub mod accounts;
pub mod entries;
pub mod invoices;
pub mod tags;

use crate::repositories::{
    accounts::AccountRepository, entries::EntriesRepository,
    invoice_relations::InvoiceRelationsRepository, invoices::InvoicesRepository,
    tags::TagRepository,
};

#[derive(Clone)]
pub struct Handler {
    invoices_repository: Arc<dyn InvoicesRepository + Send + Sync>,
    entries_repository: Arc<dyn EntriesRepository + Send + Sync>,
    accounts_repository: Arc<dyn AccountRepository + Send + Sync>,
    tags_repository: Arc<dyn TagRepository + Send + Sync>,
    invoice_relations_repository: Arc<dyn InvoiceRelationsRepository + Send + Sync>,
}

impl Handler {
    pub const fn new(
        invoices_repository: Arc<dyn InvoicesRepository + Send + Sync>,
        entries_repository: Arc<dyn EntriesRepository + Send + Sync>,
        accounts_repository: Arc<dyn AccountRepository + Send + Sync>,
        tags_repository: Arc<dyn TagRepository + Send + Sync>,
        invoice_relations_repository: Arc<dyn InvoiceRelationsRepository + Send + Sync>,
    ) -> Self {
        Self {
            invoices_repository,
            entries_repository,
            accounts_repository,
            tags_repository,
            invoice_relations_repository,
        }
    }
}
