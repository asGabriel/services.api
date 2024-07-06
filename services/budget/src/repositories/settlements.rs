use uuid::Uuid;

use crate::domains::{
    errors::Result,
    settlements::{CreateSettlement, Settlement},
};

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait SettlementRepository {
    async fn create_settlement(&self, payload: CreateSettlement) -> Result<Settlement>;
    async fn list_settlements(&self) -> Result<Vec<Settlement>>;
}

#[async_trait::async_trait]
impl SettlementRepository for SqlxRepository {
    async fn list_settlements(&self) -> Result<Vec<Settlement>> {
        let settlements = sqlx::query_as!(
            Settlement,
            r#"
                SELECT * FROM settlements
            "#
        ).fetch_all(&self.pool).await?;

        Ok(settlements)
    }

    async fn create_settlement(&self, payload: CreateSettlement) -> Result<Settlement> {
        let settlement = sqlx::query_as!(
            Settlement,
            r#"
                INSERT INTO settlements(
                    settlement_id,
                    transaction_id, 
                    installment_id,
                    paid_date,
                    paid_value,
                    discount,
                    fees,
                    attachment
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8
                ) RETURNING 
                settlement_id,
                    transaction_id, 
                    installment_id,
                    paid_date,
                    paid_value,
                    discount,
                    fees,
                    attachment,
                    created_at,
                    updated_at,
                    deleted_at
            "#,
            Uuid::new_v4(),
            payload.transaction_id,
            payload.installment_id,
            payload.paid_date,
            payload.paid_value,
            payload.discount,
            payload.fees,
            payload.attachment
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(settlement)
    }
}
