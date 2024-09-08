use finance_domains::invoices::Invoice;
use http_problems::errors::Result;

use super::Handler;

impl Handler {
    pub async fn get_operations(&self) -> Result<Vec<Invoice>> {
        let invoices = self.finance_gateway.list_invoices().await?;

        Ok(invoices)
    }
}
