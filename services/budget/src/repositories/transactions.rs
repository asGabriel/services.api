use crate::domains::{
    errors::Result,
    transactions::{report::PeriodFilter, CreateTransaction, UpdateTransaction},
};

use super::SqlxRepository;
use finance::{
    category::TransactionCategory, movement_type::MovementType, status::TransactionStatus,
    transaction::Transaction,
};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TransactionRepository {
    async fn create_transaction(&self, transaction: CreateTransaction) -> Result<Transaction>;
    async fn list_transactions(&self) -> Result<Vec<Transaction>>;
    async fn get_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>>;
    async fn delete_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>>;
    async fn update_transaction_by_id(
        &self,
        transaction: Transaction,
        payload: UpdateTransaction,
    ) -> Result<Option<Transaction>>;
    async fn update_status(
        &self,
        transaction_id: Uuid,
        status: TransactionStatus,
    ) -> Result<Option<Transaction>>;
    async fn list_transactions_by_period(&self, period: &PeriodFilter) -> Result<Vec<Transaction>>;
}

#[async_trait::async_trait]
impl TransactionRepository for SqlxRepository {
    async fn list_transactions(&self) -> Result<Vec<Transaction>> {
        let transactions = sqlx::query_as!(
            Transaction,
            r#"
            SELECT
                transaction_id, 
                movement_type as "movement_type!: MovementType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                status as "status: TransactionStatus", 
                installment_number,
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

    async fn create_transaction(&self, transaction: CreateTransaction) -> Result<Transaction> {
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
                status,
                installment_number
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            ) RETURNING 
                transaction_id, 
                movement_type as "movement_type!: MovementType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                status as "status: TransactionStatus", 
                installment_number,
                created_at, 
                updated_at, 
                deleted_at
            "#,
            Uuid::new_v4(),
            transaction.movement_type as MovementType,
            transaction.description,
            transaction.amount,
            transaction.due_date,
            transaction.category as TransactionCategory,
            transaction.account_id,
            transaction.status as TransactionStatus,
            transaction.installment_number
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
                movement_type as "movement_type!: MovementType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                status as "status: TransactionStatus", 
                installment_number,
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

    async fn delete_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            UPDATE TRANSACTIONS SET
                updated_at = now(),
                deleted_at = now()
            WHERE
                transaction_id = $1
                AND deleted_at is null
            RETURNING 
                transaction_id, 
                movement_type as "movement_type!: MovementType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                status as "status: TransactionStatus", 
                installment_number,
                created_at, 
                updated_at, 
                deleted_at
            "#,
            transaction_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(transaction)
    }

    async fn update_transaction_by_id(
        &self,
        transaction: Transaction,
        payload: UpdateTransaction,
    ) -> Result<Option<Transaction>> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            UPDATE TRANSACTIONS SET
                updated_at = now()
            WHERE 
                transaction_id = $1
            RETURNING
                transaction_id, 
                movement_type as "movement_type!: MovementType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                status as "status: TransactionStatus", 
                installment_number,
                created_at, 
                updated_at, 
                deleted_at
            "#,
            transaction.transaction_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(transaction)
    }

    async fn update_status(
        &self,
        transaction_id: Uuid,
        status: TransactionStatus,
    ) -> Result<Option<Transaction>> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            UPDATE transactions SET status = $2 WHERE transaction_id = $1
            RETURNING
                transaction_id, 
                movement_type as "movement_type!: MovementType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                status as "status: TransactionStatus", 
                installment_number,
                created_at, 
                updated_at, 
                deleted_at

            "#,
            transaction_id,
            status as TransactionStatus
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(transaction)
    }

    // TODO: revisar essa função; mover para o consolidation
    async fn list_transactions_by_period(&self, period: &PeriodFilter) -> Result<Vec<Transaction>> {
        let transactions = sqlx::query_as!(
            Transaction,
            r#"
            SELECT
                transaction_id, 
                movement_type as "movement_type!: MovementType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                installment_number,
                status as "status: TransactionStatus", 
                created_at, 
                updated_at, 
                deleted_at
            FROM transactions
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(transactions)
    }
}
