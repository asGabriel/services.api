use crate::domains::{
    accounts::{Account, AccountType, Bank, CreateAccount},
    errors::Result,
};

use super::SqlxRepository;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait AccountRepository {
    async fn list_accounts(&self) -> Result<Vec<Account>>;
    async fn create_account(&self, account: CreateAccount) -> Result<Account>;
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
}
