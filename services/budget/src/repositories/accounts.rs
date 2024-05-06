use crate::domains::{
    accounts::{Account, Bank},
    errors::Result,
};

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait AccountRepository {
    async fn list_accounts(&self) -> Result<Vec<Account>>;
}

#[async_trait::async_trait]
impl AccountRepository for SqlxRepository {
    async fn list_accounts(&self) -> Result<Vec<Account>> {
        let accounts = sqlx::query_as!(
            Account,
            r#"
            SELECT 
                account_id,
                name,
                bank_name as "bank_name!: Bank",
                owner,
                created_at,
                updated_at,
                deleted_at
            FROM ACCOUNTS
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(accounts)
    }
}
