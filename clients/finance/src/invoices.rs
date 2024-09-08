use finance_domains::Invoice;
use http_problems::errors::Result;
use reqwest::Client;

#[derive(Debug)]
pub struct FinanceClient {
    client: Client,
    url: String,
}

impl FinanceClient {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let base_url = std::env::var("BASE_URL").expect("missing BASE_URL");

        FinanceClient {
            client: Client::new(),
            url: base_url,
        }
    }
}

#[async_trait::async_trait]
pub trait InvoicesGateway {
    async fn list_invoices(&self) -> Result<Vec<Invoice>>;
}

#[async_trait::async_trait]
impl InvoicesGateway for FinanceClient {
    async fn list_invoices(&self) -> Result<Vec<Invoice>> {
        let url = format!("{}/invoices", &self.url);

        let response = self.client.get(url).send().await?;
        let invoices = response.json().await?;

        Ok(invoices)
    }
}
