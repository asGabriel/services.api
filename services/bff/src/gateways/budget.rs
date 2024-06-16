use finance::transaction::Transaction;

use crate::domains::errors::Result;

pub struct ApiBudgetGateway {
    http_client: reqwest::Client,
}

impl Default for ApiBudgetGateway {
    fn default() -> Self {
        Self {
            http_client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
pub trait BudgetGateway {
    async fn transactions(&self) -> Result<Vec<Transaction>>;
}

#[async_trait::async_trait]
impl BudgetGateway for ApiBudgetGateway {
    async fn transactions(&self) -> Result<Vec<Transaction>> {
        let transactions = self
            .http_client
            .get("http://localhost:8000/transactions")
            .send()
            .await?
            .json::<Vec<Transaction>>().await?;

        Ok(transactions)
    }
}
