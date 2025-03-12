use crate::domains::accounts::Bank;
use http_problems::Result;
use uuid::Uuid;

use crate::domains::accounts::Account;

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait AccountRepository {
    async fn list_accounts(&self) -> Result<Vec<Account>>;
    async fn create_account(&self, account: Account) -> Result<Account>;
    async fn get_account_by_id(&self, account_id: Uuid) -> Result<Option<Account>>;
}

#[async_trait::async_trait]
impl AccountRepository for SqlxRepository {
    async fn list_accounts(&self) -> Result<Vec<Account>> {
        let account = sqlx::query_as!(
            Account,
            r#"
                SELECT 
                    account_id,
                    bank_name as "bank_name!: Bank",
                    owner,
                    identification,
                    created_at, 
                    updated_at,
                    deleted_at
                FROM accounts
                WHERE
                    deleted_at is null
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(account)
    }

    async fn create_account(&self, account: Account) -> Result<Account> {
        let account = sqlx::query_as!(
            Account,
            r#"
                INSERT INTO accounts (
                    account_id,
                    bank_name,
                    owner,
                    created_at,
                    updated_at,
                    deleted_at,
                    identification
                ) 
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING
                account_id,
                    bank_name as "bank_name!: Bank",
                    owner,
                    created_at, 
                    updated_at,
                    deleted_at,
                    identification
            "#,
            account.account_id,
            account.bank_name as Bank,
            account.owner,
            account.created_at,
            account.updated_at,
            account.deleted_at,
            account.identification
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(account)
    }

    async fn get_account_by_id(&self, account_id: Uuid) -> Result<Option<Account>> {
        let account = sqlx::query_as!(
            Account,
            r#"
                SELECT 
                    account_id,
                    bank_name as "bank_name!: Bank",
                    owner,
                    identification,
                    created_at, 
                    updated_at,
                    deleted_at
                FROM accounts
                WHERE
                    account_id = $1
                    and deleted_at is null
            "#,
            account_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(account)
    }
}
