use crate::domains::{accounts::Account, errors::Result};

use super::Handler;

impl Handler {
    pub async fn list_accounts(&self) -> Result<Vec<Account>> {
        self.account_repository.list_accounts().await
    }
}
