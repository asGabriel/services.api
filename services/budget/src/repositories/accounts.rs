use crate::domains::{
    accounts::{Account, AccountType, Bank, CreateAccount, UpdateAccount},
    errors::Result,
};

use super::SqlxRepository;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait AccountRepository {
    async fn list_accounts(&self) -> Result<Vec<Account>>;
    async fn create_account(&self, account: CreateAccount) -> Result<Account>;
    async fn get_account_by_id(&self, account_id: Uuid) -> Result<Option<Account>>;
    async fn update_account_by_id(
        &self,
        account: Account,
        payload: UpdateAccount,
    ) -> Result<Option<Account>>;
    async fn delete_account_by_id(&self, account_id: Uuid) -> Result<Option<Account>>;
}

#[async_trait::async_trait]
impl AccountRepository for SqlxRepository {
    async fn list_accounts(&self) -> Result<Vec<Account>> {
        let accounts = sqlx::query_as!(
            Account,
            r#"
            SELECT 
                account_id,
                bank_name as "bank_name!: Bank",
                owner,
                account_type as "account_type!: AccountType",
                created_at,
                updated_at,
                deleted_at
            FROM accounts
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(accounts)
    }

    async fn create_account(&self, account: CreateAccount) -> Result<Account> {
        let account = sqlx::query_as!(
            Account,
            r#"
            INSERT INTO accounts (
                account_id, bank_name, owner, account_type
            ) VALUES (
                $1, $2, $3, $4
            ) RETURNING
                account_id,
                bank_name as "bank_name!: Bank",
                owner,
                account_type as "account_type!: AccountType",
                created_at,
                updated_at,
                deleted_at
            "#,
            Uuid::new_v4(),
            account.bank_name as Bank,
            account.owner,
            account.account_type as AccountType
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
                account_type as "account_type!: AccountType",
                created_at, 
                updated_at, 
                deleted_at 
            FROM accounts WHERE account_id = $1
            "#,
            account_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(account)
    }

    async fn update_account_by_id(
        &self,
        account: Account,
        payload: UpdateAccount,
    ) -> Result<Option<Account>> {
        let account = sqlx::query_as!(
            Account,
            r#"
            UPDATE accounts SET
                updated_at = now(),
                bank_name = $2,
                owner = $3,
                account_type = $4
            WHERE
                account_id = $1 AND deleted_at is null
            RETURNING
                account_id,
                bank_name as "bank_name!: Bank",
                owner,
                account_type as "account_type!: AccountType",
                created_at, 
                updated_at, 
                deleted_at 
            "#,
            account.account_id,
            payload.bank_name.unwrap_or(account.bank_name) as Bank,
            payload.owner.unwrap_or(account.owner),
            payload.account_type.unwrap_or(account.account_type) as AccountType
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(account)
    }

    async fn delete_account_by_id(&self, account_id: Uuid) -> Result<Option<Account>> {
        let account = sqlx::query_as!(
            Account,
            r#"
            UPDATE accounts SET
                updated_at = now(),
                deleted_at = now()
            WHERE
                account_id = $1 and deleted_at is null
            RETURNING
                account_id,
                bank_name as "bank_name!: Bank",
                owner,
                account_type as "account_type!: AccountType",
                created_at, 
                updated_at, 
                deleted_at 
            "#,
            account_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(account)
    }
}
