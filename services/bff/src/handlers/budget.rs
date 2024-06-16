use std::sync::Arc;

use finance::transaction::Transaction;

use crate::{domains::errors::Result, gateways::budget::BudgetGateway};

pub type DynBudgetHandler = dyn BudgetHandler + Send + Sync;

pub struct BudgetHandlerImpl {
    pub budget_gateway: Arc<dyn BudgetGateway + Send + Sync>
}

#[async_trait::async_trait]
pub trait BudgetHandler {
    async fn get_transactions(&self) -> Result<Vec<Transaction>>;
}

#[async_trait::async_trait]
impl BudgetHandler for BudgetHandlerImpl {
    async fn get_transactions(&self) -> Result<Vec<Transaction>> {
        let transaction = self.budget_gateway.transactions().await?;

        Ok(transaction)
    }
}
