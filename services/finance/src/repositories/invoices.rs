use http_problems::Result;
use uuid::Uuid;

use crate::{domains::invoices::Invoice, handlers::invoices::InvoiceReferenceParams};

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait InvoicesRepository {
    async fn list_invoices(&self) -> Result<Vec<Invoice>>;
    async fn get_invoice_by_id(&self, invoice_id: Uuid) -> Result<Option<Invoice>>;
    async fn get_main_invoice_by_reference(
        &self,
        params: InvoiceReferenceParams,
    ) -> Result<Option<Invoice>>;
    async fn create_invoice(&self, invoice: Invoice) -> Result<Invoice>;
    async fn update_invoice_by_id(&self, invoice: Invoice) -> Result<Option<Invoice>>;
}

#[async_trait::async_trait]
impl InvoicesRepository for SqlxRepository {
    async fn update_invoice_by_id(&self, invoice: Invoice) -> Result<Option<Invoice>> {
        let invoice = sqlx::query_as!(
            Invoice,
            r#"
                UPDATE 
                    invoices
                SET
                    title = $2,
                    month = $3,
                    year = $4
                WHERE
                    invoice_id = $1
                RETURNING
                    *
            "#,
            invoice.invoice_id,
            invoice.title,
            invoice.month,
            invoice.year
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(invoice)
    }

    async fn get_main_invoice_by_reference(
        &self,
        params: InvoiceReferenceParams,
    ) -> Result<Option<Invoice>> {
        let invoice = sqlx::query_as!(
            Invoice,
            r#"
                SELECT
                    invoice_id,
                    title,
                    month,
                    year,
                    is_main,
                    created_at,
                    updated_at,
                    deleted_at
                FROM invoices
                WHERE
                    year = $1
                    AND month = $2
                    AND is_main IS true
            "#,
            params.year as i16,
            params.month as i32
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(invoice)
    }

    async fn create_invoice(&self, invoice: Invoice) -> Result<Invoice> {
        let invoice = sqlx::query_as!(
            Invoice,
            r#"
                INSERT INTO invoices (
                    invoice_id,
                    title,
                    month,
                    year,
                    is_main,
                    created_at,
                    updated_at,
                    deleted_at
                    ) VALUES (
                        $1, $2, $3, $4, $5, $6, $7, $8
                    ) RETURNING
                        invoice_id,
                        title,
                        month,
                        year,
                        is_main,
                        created_at,
                        updated_at,
                        deleted_at
                
            "#,
            invoice.invoice_id,
            invoice.title,
            invoice.month,
            invoice.year,
            invoice.is_main,
            invoice.created_at,
            invoice.updated_at,
            invoice.deleted_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(invoice)
    }

    async fn get_invoice_by_id(&self, invoice_id: Uuid) -> Result<Option<Invoice>> {
        let invoice = sqlx::query_as!(
            Invoice,
            r#"
                SELECT
                    invoice_id,
                    title,
                    month,
                    year,
                    is_main,
                    created_at,
                    updated_at,
                    deleted_at
                FROM invoices
                WHERE
                    invoice_id = $1
            "#,
            invoice_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(invoice)
    }

    async fn list_invoices(&self) -> Result<Vec<Invoice>> {
        let invoices = sqlx::query_as!(
            Invoice,
            r#"
                SELECT
                    invoice_id,
                    title,
                    month,
                    year,
                    is_main,
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
