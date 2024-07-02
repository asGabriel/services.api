use std::collections::BTreeMap;

use chrono::NaiveDate;
use finance::{category::Category, frequency::Frequency};
use uuid::Uuid;

use crate::domains::{
    errors::Result,
    recurrence_transactions::{
        CreateRecurrenceTransaction, GeneratedTransaction, RecurrenceTransaction,
    },
};

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait RecurrenceTransactionRepository {
    async fn create_recurrence_transaction(
        &self,
        payload: CreateRecurrenceTransaction,
    ) -> Result<RecurrenceTransaction>;
    async fn list_recurrence_transactions(&self) -> Result<Vec<RecurrenceTransaction>>;
    async fn list_generated_transactions_from_recurrence(
        &self,
    ) -> Result<BTreeMap<NaiveDate, GeneratedTransaction>>;
}

#[async_trait::async_trait]
impl RecurrenceTransactionRepository for SqlxRepository {
    async fn list_recurrence_transactions(&self) -> Result<Vec<RecurrenceTransaction>> {
        let result = sqlx::query_as!(
            RecurrenceTransaction,
            r#"
                SELECT
                    recurrence_transaction_id,
                    account_id,
                    description,
                    amount,
                    frequency as "frequency: Frequency",
                    is_active,
                    created_at,
                    updated_at,
                    deleted_at,
                    category as "category: Category",
                    reference,
                    start_date
                FROM recurrence_transactions
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }

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
                    category,
                    start_date
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8
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
                deleted_at,
                start_date
            "#,
            Uuid::new_v4(),
            payload.account_id,
            payload.description,
            payload.amount,
            payload.frequency as Frequency,
            payload.reference,
            payload.category as Category,
            payload.start_date
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn list_generated_transactions_from_recurrence(
        &self,
    ) -> Result<BTreeMap<NaiveDate, GeneratedTransaction>> {
        let active_generated_transactions = sqlx::query!(
            r#"
            SELECT
                gt.id, gt.recurrence_transaction_id, gt.transaction_id, gt.created_at, gt.deleted_at, t.due_date
            FROM generated_transactions gt
            INNER JOIN recurrence_transactions rt ON gt.recurrence_transaction_id = rt.recurrence_transaction_id
            INNER JOIN transactions t ON gt.transaction_id = t.transaction_id
            WHERE
                rt.is_active = true AND
                t.deleted_at IS NULL
            "#
        ).fetch_all(&self.pool).await?;

        let mut generated_transactions: BTreeMap<NaiveDate, GeneratedTransaction> = BTreeMap::new();
        for record in active_generated_transactions {
            generated_transactions.insert(
                record.due_date,
                GeneratedTransaction {
                    id: record.id,
                    recurrence_transaction_id: record.recurrence_transaction_id,
                    transaction_id: record.transaction_id,
                    created_at: record.created_at,
                    deleted_at: record.deleted_at,
                },
            );
        }

        Ok(generated_transactions)
    }
}
