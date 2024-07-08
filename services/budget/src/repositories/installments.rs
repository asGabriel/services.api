use finance::{
    installment::{Installment, PartialInstallment},
    status::TransactionStatus,
};
use uuid::Uuid;

use crate::domains::errors::Result;

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait InstallmentRepository {
    async fn create_installment(&self, payload: &PartialInstallment) -> Result<Installment>;
    async fn get_installment_by_id(&self, id: Uuid) -> Result<Option<Installment>>;
}

#[async_trait::async_trait]
impl InstallmentRepository for SqlxRepository {
    async fn create_installment(&self, payload: &PartialInstallment) -> Result<Installment> {
        let installment = sqlx::query_as!(
            Installment,
            r#"
            INSERT INTO installments (
                installment_id,
                transaction_id, 
                installment_number, 
                due_date, 
                value, 
                status,
                total_installment
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7
            ) RETURNING
                installment_id,
                transaction_id,
                installment_number,
                total_installment,
                due_date,
                value,
                status as "status!: TransactionStatus",
                created_at,
                updated_at,
                deleted_at
            "#,
            Uuid::new_v4(),
            payload.transaction_id,
            payload.params.installment_number,
            payload.due_date,
            payload.value,
            payload.status as TransactionStatus,
            payload.params.total_installment
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(installment)
    }

    async fn get_installment_by_id(&self, installment_id: Uuid) -> Result<Option<Installment>> {
        let installment = sqlx::query_as!(
            Installment,
            r#"
            SELECT  
                installment_id,
                transaction_id,
                installment_number,
                total_installment,
                due_date,
                value,
                status as "status!: TransactionStatus",
                created_at,
                updated_at,
                deleted_at 
            FROM installments
            WHERE
                installment_id = $1
            "#,
            installment_id
        ).fetch_optional(&self.pool).await?;

        Ok(installment)
    }
}
