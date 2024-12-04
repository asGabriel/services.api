use http_problems::{Error, Result};
use uuid::Uuid;

use crate::domains::accounts::{Account, CreateAccountPayload};

use super::Handler;

impl Handler {
    pub async fn create_account(&self, payload: CreateAccountPayload) -> Result<Account> {
        let new_account: Account = payload.into();

        self.accounts_repository.create_account(new_account).await
    }

    pub async fn get_account_by_id(&self, account_id: Uuid) -> Result<Account> {
        self.accounts_repository
            .get_account_by_id(account_id)
            .await?
            .ok_or(Error::NotFoundError(format!(
                "Account id {account_id} not found"
            )))
    }

    pub async fn list_accounts(&self) -> Result<Vec<Account>> {
        self.accounts_repository.list_accounts().await
    }
}
