use finance_domains::{entries::Entry, invoices::Invoice};
use http_problems::errors::Result;

use crate::FinanceClient;

#[async_trait::async_trait]
pub trait FinanceGateway {
    async fn list_invoices(&self) -> Result<Vec<Invoice>>;
    async fn list_entries(&self) -> Result<Vec<Entry>>;
}

#[async_trait::async_trait]
impl FinanceGateway for FinanceClient {
    async fn list_invoices(&self) -> Result<Vec<Invoice>> {
        let url = format!("{}/invoices", &self.url);

        let response = self.client.get(url).send().await?;
        let invoices = response.json().await?;

        Ok(invoices)
    }

    async fn list_entries(&self) -> Result<Vec<Entry>> {
        let url = format!("{}/invoices", &self.url);

        let response = self.client.get(url).send().await?;
        let entries = response.json().await?;

        Ok(entries)
    }
}
