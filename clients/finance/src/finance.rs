use finance_domains::{entries::Entry, invoices::Invoice};
use http_problems::errors::Result;
use uuid::Uuid;

use crate::FinanceClient;

#[async_trait::async_trait]
pub trait FinanceGateway {
    async fn list_invoices(&self) -> Result<Vec<Invoice>>;
    async fn get_invoice_by_id(&self, invoice_id: Uuid) -> Result<Invoice>;
    async fn list_entries(&self) -> Result<Vec<Entry>>;
    async fn list_entries_by_invoice_id(&self, invoice_id: Uuid) -> Result<Vec<Entry>>;
}

#[async_trait::async_trait]
impl FinanceGateway for FinanceClient {
    async fn get_invoice_by_id(&self, invoice_id: Uuid) -> Result<Invoice> {
        let url = format!("{}/invoices/{}", &self.url, invoice_id);

        let response = self.client.get(url).send().await?;

        let result = response.json().await?;

        Ok(result)
    }

    async fn list_invoices(&self) -> Result<Vec<Invoice>> {
        let url = format!("{}/invoices", &self.url);

        let response = self.client.get(url).send().await?;
        let invoices = response.json().await?;

        Ok(invoices)
    }

    async fn list_entries(&self) -> Result<Vec<Entry>> {
        let url = format!("{}/entries", &self.url);

        let response = self.client.get(url).send().await?;
        let entries = response.json().await?;

        Ok(entries)
    }

    async fn list_entries_by_invoice_id(&self, invoice_id: Uuid) -> Result<Vec<Entry>> {
        let url = format!("{}/entries/invoice/{}", &self.url, invoice_id);

        let response = self.client.get(url).send().await?;
        let entries = response.json().await?;

        Ok(entries)
    }
}
