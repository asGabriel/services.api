use http_problems::Result;

use crate::domains::invoice_relations::InvoiceRelations;

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait InvoiceRelationsRepository {
    async fn create_relations(&self, relations: InvoiceRelations) -> Result<InvoiceRelations>;
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
}
