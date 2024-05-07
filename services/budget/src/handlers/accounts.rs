use uuid::Uuid;

use crate::domains::{
    accounts::{Account, CreateAccount},
    errors::{Error, Result},
};

use super::Handler;

impl Handler {
    pub async fn list_accounts(&self) -> Result<Vec<Account>> {
        self.account_repository.list_accounts().await
    }

    pub async fn create_account(&self, payload: CreateAccount) -> Result<Account> {
        self.account_repository.create_account(payload).await
    }

    pub async fn get_account_by_id(&self, account_id: Uuid) -> Result<Account> {
        self.account_repository.get_account_by_id(account_id).await?.ok_or(Error::AccountNotFound(account_id))
    }
}