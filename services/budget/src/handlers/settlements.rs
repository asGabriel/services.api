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
        let _ = self.get_transaction_by_id(query.transaction_id).await?;

        if let Some(installment_id) = query.installment_id {
            let _ = self.get_installment_by_id(installment_id).await?;
        }

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
