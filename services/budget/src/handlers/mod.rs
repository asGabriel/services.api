use std::sync::Arc;

use crate::repositories::{
    accounts::AccountRepository, installments::InstallmentRepository,
    recurrences::RecurrenceRepository, settlements::SettlementRepository,
    transactions::TransactionRepository,
};

pub mod accounts;
pub mod installments;
pub mod recurrences;
pub mod settlements;
pub mod transactions;

#[derive(Clone)]
pub struct Handler {
    transaction_repository: Arc<dyn TransactionRepository + Send + Sync>,
    account_repository: Arc<dyn AccountRepository + Send + Sync>,
    installment_repository: Arc<dyn InstallmentRepository + Send + Sync>,
    settlement_repository: Arc<dyn SettlementRepository + Send + Sync>,
    recurrence_repository: Arc<dyn RecurrenceRepository + Send + Sync>,
}

impl Handler {
    pub const fn new(
        transactions_repository: Arc<dyn TransactionRepository + Send + Sync>,
        accounts_repository: Arc<dyn AccountRepository + Send + Sync>,
        installment_repository: Arc<dyn InstallmentRepository + Send + Sync>,
        settlement_repository: Arc<dyn SettlementRepository + Send + Sync>,
        recurrence_repository: Arc<dyn RecurrenceRepository + Send + Sync>,
    ) -> Self {
        Self {
            transaction_repository: transactions_repository,
            account_repository: accounts_repository,
            installment_repository: installment_repository,
            settlement_repository: settlement_repository,
            recurrence_repository: recurrence_repository,
        }
    }
}
