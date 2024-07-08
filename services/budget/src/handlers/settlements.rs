use crate::domains::{
    errors::Result,
    settlements::{CreateSettlement, Settlement, SettlementParams},
};

use super::Handler;

impl Handler {
    pub async fn create_settlement(
        &self,
        payload: CreateSettlement,
        query: SettlementParams,
    ) -> Result<Settlement> {
        let new_settlement = Settlement::new_from_payload(payload, query);

        let settlement = self
            .settlement_repository
            .create_settlement(new_settlement)
            .await?;

        Ok(settlement)
    }

    pub async fn list_settlements(&self) -> Result<Vec<Settlement>> {
        let settlements = self.settlement_repository.list_settlements().await?;

        Ok(settlements)
    }
}
