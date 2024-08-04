use crate::domains::{
    errors::Result,
    transactions::{Category, MovementType, Transaction, TransactionStatus},
};

use mockall::automock;

use super::SqlxRepository;
use uuid::Uuid;

#[automock]
#[async_trait::async_trait]
pub trait TransactionRepository {
    async fn create_transaction(&self, transaction: Transaction) -> Result<Transaction>;
    async fn list_transactions(&self) -> Result<Vec<Transaction>>;
    async fn get_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>>;
    async fn delete_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>>;
    async fn update_transaction_by_id(
        &self,
        transaction: Transaction,
    ) -> Result<Option<Transaction>>;
    async fn update_status(
        &self,
        transaction_id: Uuid,
        status: TransactionStatus,
    ) -> Result<Option<Transaction>>;
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
                value, 
                due_date, 
                category as "category: Category", 
                account_id, 
                status as "status: TransactionStatus", 
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

    async fn create_transaction(&self, transaction: Transaction) -> Result<Transaction> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            INSERT INTO TRANSACTIONS (
                transaction_id,
                movement_type,
                description,
                value,
                due_date,
                category,
                account_id,
                status
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8
            ) RETURNING 
                transaction_id, 
                movement_type as "movement_type!: MovementType",
                description, 
                value, 
                due_date, 
                category as "category: Category", 
                account_id, 
                status as "status: TransactionStatus", 
                created_at, 
                updated_at, 
                deleted_at
            "#,
            transaction.transaction_id,
            transaction.movement_type as MovementType,
            transaction.description,
            transaction.value.normalized(),
            transaction.due_date,
            transaction.category as Category,
            transaction.account_id,
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
                movement_type as "movement_type!: MovementType",
                description, 
                value, 
                due_date, 
                category as "category: Category", 
                account_id, 
                status as "status: TransactionStatus", 
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
                value, 
                due_date, 
                category as "category: Category", 
                account_id, 
                status as "status: TransactionStatus", 
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
    ) -> Result<Option<Transaction>> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            UPDATE TRANSACTIONS SET
                movement_type = $2,
                description = $3,
                value = $4,
                due_date = $5,
                category = $6,
                account_id = $7,
                updated_at = $8
            WHERE 
                transaction_id = $1
            RETURNING
                transaction_id, 
                movement_type as "movement_type!: MovementType",
                description, 
                value, 
                due_date, 
                category as "category: Category", 
                account_id, 
                status as "status: TransactionStatus", 
                created_at, 
                updated_at, 
                deleted_at
            "#,
            transaction.transaction_id,
            transaction.movement_type as MovementType,
            transaction.description,
            transaction.value.normalized(),
            transaction.due_date,
            transaction.category as Category,
            transaction.account_id,
            transaction.updated_at
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
                value, 
                due_date, 
                category as "category: Category", 
                account_id, 
                status as "status: TransactionStatus", 
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
}
