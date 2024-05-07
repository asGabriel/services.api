use uuid::Uuid;

use crate::domains::{
    accounts::{Account, CreateAccount, UpdateAccount},
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
        self.account_repository
            .get_account_by_id(account_id)
            .await?
            .ok_or(Error::AccountNotFound(account_id))
    }

    pub async fn delete_account_by_id(&self, account_id: Uuid) -> Result<Account> {
        self.get_account_by_id(account_id).await?;

        self.account_repository
            .delete_account_by_id(account_id)
            .await?
            .ok_or(Error::AccountAlreadyDeleted(account_id))
    }

    pub async fn update_account_by_id(
        &self,
        account_id: Uuid,
        payload: UpdateAccount,
    ) -> Result<Account> {
        let result = self.get_account_by_id(account_id).await?;

        self.account_repository
            .update_account_by_id(result, payload)
            .await?
            .ok_or(Error::AccountAlreadyDeleted(account_id))
    }
}
