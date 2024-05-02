use std::sync::Arc;

use crate::repositories::transactions::TransactionRepository;

pub mod transactions;

#[derive(Clone)]
pub struct Handler {
    transactions_repository: Arc<dyn TransactionRepository + Send + Sync>,
}

impl Handler {
    pub const fn new(
        transactions_repository: Arc<dyn TransactionRepository + Send + Sync>,
    ) -> Self {
        Self {
            transactions_repository,
        }
    }
}
