use crate::domains::{
    accounts::{Account, CreateAccount},
    errors::Result,
};

use super::Handler;

impl Handler {
    pub async fn list_accounts(&self) -> Result<Vec<Account>> {
        self.account_repository.list_accounts().await
    }

    pub async fn create_account(&self, payload: CreateAccount) -> Result<Account> {
        self.account_repository.create_account(payload).await
    }
}
