use finance::{category::Category, frequency::Frequency};
use uuid::Uuid;

use crate::domains::{
    errors::Result,
    recurrence_transactions::{CreateRecurrenceTransaction, RecurrenceTransaction},
};

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait RecurrenceTransactionRepository {
    async fn create_recurrence_transaction(
        &self,
        payload: CreateRecurrenceTransaction,
    ) -> Result<RecurrenceTransaction>;
}

#[async_trait::async_trait]
impl RecurrenceTransactionRepository for SqlxRepository {
    async fn create_recurrence_transaction(
        &self,
        payload: CreateRecurrenceTransaction,
    ) -> Result<RecurrenceTransaction> {
        let result = sqlx::query_as!(
            RecurrenceTransaction,
            r#"
                INSERT INTO recurrence_transactions(
                    recurrence_transaction_id,
                    account_id,
                    description,
                    amount,
                    frequency,
                    reference,
                    category
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7
                ) RETURNING 
                recurrence_transaction_id,
                account_id,
                description,
                amount,
                frequency as "frequency!: Frequency",
                reference,
                is_active,
                category as "category!: Category",
                created_at,
                updated_at,
                deleted_at
            "#,
            Uuid::new_v4(),
            payload.account_id,
            payload.description,
            payload.amount,
            payload.frequency as Frequency,
            payload.reference,
            payload.category as Category
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }
}
