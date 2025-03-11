use http_problems::Result;

use crate::domains::{invoice_relations::InvoiceRelations, invoices::Invoice};

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait InvoiceRelationsRepository {
    async fn create_relations(&self, relations: InvoiceRelations) -> Result<InvoiceRelations>;
    async fn list_related_invoices(&self, invoice: &Invoice) -> Result<Vec<InvoiceRelations>>;
}

#[async_trait::async_trait]
impl InvoiceRelationsRepository for SqlxRepository {
    async fn create_relations(&self, payload: InvoiceRelations) -> Result<InvoiceRelations> {
        let relation = sqlx::query_as!(
            InvoiceRelations,
            r#"
                INSERT INTO invoice_relations (parent_invoice_id, child_invoice_id)
                VALUES ($1, $2)
                RETURNING *
            "#,
            payload.parent_invoice_id,
            payload.child_invoice_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(relation)
    }

    async fn list_related_invoices(&self, invoice: &Invoice) -> Result<Vec<InvoiceRelations>> {
        let relation = sqlx::query_as!(
            InvoiceRelations,
            r#"
                SELECT 
                    * 
                FROM invoice_relations 
                WHERE 
                    parent_invoice_id = $1
            "#,
            invoice.invoice_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(relation)
    }
}
