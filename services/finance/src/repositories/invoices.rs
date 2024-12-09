use http_problems::Result;
use uuid::Uuid;

use crate::domains::invoices::{Invoice, InvoicePayload};

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait InvoicesRepository {
    async fn list_invoices(&self) -> Result<Vec<Invoice>>;
    async fn get_invoice_by_id(&self, invoice_id: Uuid) -> Result<Option<Invoice>>;
    async fn get_invoice_by_reference(&self, params: &InvoicePayload) -> Result<Option<Invoice>>;
    async fn create_invoice(&self, invoice: Invoice) -> Result<Invoice>;
}

#[async_trait::async_trait]
impl InvoicesRepository for SqlxRepository {
    async fn get_invoice_by_reference(&self, params: &InvoicePayload) -> Result<Option<Invoice>> {
        let invoice = sqlx::query_as!(
            Invoice,
            r#"
                SELECT
                    invoice_id,
                    title,
                    month,
                    year,
                    is_parent,
                    created_at,
                    updated_at,
                    deleted_at
                FROM invoices
                WHERE
                    year = $1
                    AND month = $2
                    AND is_parent IS TRUE
            "#,
            params.year,
            params.month
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
                    is_parent,
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
                        is_parent,
                        created_at,
                        updated_at,
                        deleted_at
                
            "#,
            invoice.invoice_id,
            invoice.title,
            invoice.month,
            invoice.year,
            invoice.is_parent,
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
                    is_parent,
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
                    is_parent,
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
