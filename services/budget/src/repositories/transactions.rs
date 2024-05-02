use crate::domains::{
    errors::Result,
    transactions::{
        CreateTransactionDto, Transaction, TransactionCategory, TransactionRecurrency,
        TransactionStatus, TransactionType,
    },
};

use super::SqlxRepository;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TransactionRepository {
    async fn create_transaction(&self, transaction: CreateTransactionDto) -> Result<Transaction>;
    async fn list_transactions(&self) -> Result<Vec<Transaction>>;
    async fn get_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>>;
}

#[async_trait::async_trait]
impl TransactionRepository for SqlxRepository {
    async fn list_transactions(&self) -> Result<Vec<Transaction>> {
        let transactions = sqlx::query_as!(
            Transaction,
            r#"
            SELECT
                transaction_id, 
                movement_type as "movement_type!: TransactionType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                recurring, 
                recurrence_frequency as "recurrence_frequency: TransactionRecurrency", 
                recurrence_duration_months, 
                status as "status: TransactionStatus", 
                note, 
                created_at, 
                updated_at, 
                deleted_at
            FROM TRANSACTIONS
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(transactions)
    }

    async fn create_transaction(&self, transaction: CreateTransactionDto) -> Result<Transaction> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            INSERT INTO TRANSACTIONS (
                transaction_id,
                movement_type,
                description,
                amount,
                due_date,
                category,
                account_id,
                recurring,
                recurrence_frequency,
                recurrence_duration_months,
                status
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            ) RETURNING 
                transaction_id, 
                movement_type as "movement_type!: TransactionType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                recurring, 
                recurrence_frequency as "recurrence_frequency: TransactionRecurrency", 
                recurrence_duration_months, 
                status as "status: TransactionStatus", 
                note, 
                created_at, 
                updated_at, 
                deleted_at
            "#,
            Uuid::new_v4(),
            transaction.movement_type as TransactionType,
            transaction.description,
            transaction.amount,
            transaction.due_date,
            transaction.category as TransactionCategory,
            transaction.account_id,
            transaction.recurring,
            transaction.recurrence_frequency as TransactionRecurrency,
            transaction.recurrence_duration_months,
            transaction.status as TransactionStatus
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(transaction)
    }

    async fn get_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            SELECT
                transaction_id, 
                movement_type as "movement_type!: TransactionType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                recurring, 
                recurrence_frequency as "recurrence_frequency: TransactionRecurrency", 
                recurrence_duration_months, 
                status as "status: TransactionStatus", 
                note, 
                created_at, 
                updated_at, 
                deleted_at
            FROM TRANSACTIONS
            WHERE transaction_id = $1
            "#,
            transaction_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(transaction)
    }
}
