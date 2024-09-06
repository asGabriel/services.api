use crate::domains::{errors::Result, invoices::Invoice};

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait InvoicesRepository {
    async fn list_invoices(&self) -> Result<Vec<Invoice>>;
}

#[async_trait::async_trait]
impl InvoicesRepository for SqlxRepository {
    async fn list_invoices(&self) -> Result<Vec<Invoice>> {
        let invoices = sqlx::query_as!(
            Invoice,
            r#"
                SELECT
                    invoice_id,
                    title,
                    month,
                    year,
                    created_at,
                    updated_at,
                    deleted_at
                FROM invoices
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(invoices)
    }
}
