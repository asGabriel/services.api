use crate::domains::{
    errors::Result,
    settlements::{CreateSettlement, Settlement},
};

use super::Handler;

impl Handler {
    pub async fn create_settlement(&self, payload: CreateSettlement) -> Result<Settlement> {
        let settlement = self
            .settlement_repository
            .create_settlement(payload)
            .await?;

        Ok(settlement)
    }

    pub async fn list_settlements(&self) -> Result<Vec<Settlement>> {
        let settlements = self.settlement_repository.list_settlements().await?;

        Ok(settlements)
    }
}
