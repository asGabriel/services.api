use finance_domains::Invoice;
use http_problems::errors::Result;

use super::Handler;

impl Handler {
    pub async fn list_invoices(&self) -> Result<Vec<Invoice>> {
        let invoices = self.invoices_gateway.list_invoices().await?;

        Ok(invoices)
    }
}
