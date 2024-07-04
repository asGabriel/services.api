use finance::{installment::PartialInstallment, status::TransactionStatus};
use uuid::Uuid;

use crate::domains::{
    errors::Result,
    installments::{Installment, InstallmentParams},
    transactions::MonthReference,
};

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait InstallmentRepository {
    async fn create_installment(
        &self,
        payload: &PartialInstallment,
        params: &InstallmentParams,
        step: i16,
    ) -> Result<Installment>;
}

#[async_trait::async_trait]
impl InstallmentRepository for SqlxRepository {
    async fn create_installment(
        &self,
        payload: &PartialInstallment,
        params: &InstallmentParams,
        step: i16,
    ) -> Result<Installment> {
        let installment = sqlx::query_as!(
            Installment,
            r#"
            INSERT INTO installments (
                installment_id,
                transaction_id, 
                installment_number, 
                due_date, 
                value, 
                status 
            ) VALUES (
                $1, $2, $3, $4, $5, $6
            ) RETURNING
                installment_id,
                transaction_id,
                installment_number,
                due_date,
                value,
                status as "status!: TransactionStatus",
                created_at,
                updated_at,
                deleted_at
            "#,
            Uuid::new_v4(),
            payload.transaction_id,
            step,
            payload.due_date,
            payload.value,
            payload.status as TransactionStatus
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(installment)
    }
}
