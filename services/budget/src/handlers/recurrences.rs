use crate::domains::{errors::Result, recurrences::Recurrences};

use super::Handler;

impl Handler {
    pub async fn list_recurrences(&self) -> Result<Vec<Recurrences>> {
        let recurrences = self.recurrence_repository.list_recurrences().await?;

        Ok(recurrences)
    }
}
