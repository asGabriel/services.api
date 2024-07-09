use crate::domains::{
    errors::{Error, Result},
    settlements::{CreateSettlement, Settlement, SettlementParams},
    transactions::TransactionStatus,
};

use super::Handler;

impl Handler {
    pub async fn create_settlement(
        &self,
        payload: CreateSettlement,
        query: SettlementParams,
    ) -> Result<Settlement> {
        let transaction = self.get_transaction_by_id(query.transaction_id).await?;

        if transaction.is_finished() {
            return Err(Error::TransactionFinished(transaction.transaction_id));
        }

        if let Some(installment_id) = query.installment_id.clone() {
            let installment = self.get_installment_by_id(installment_id).await?;

            if installment.is_finished() {
                return Err(Error::TransactionFinished(installment_id));
            }
        }

        let new_settlement = Settlement::new_from_payload(payload, query);

        let settlement = self
            .settlement_repository
            .create_settlement(new_settlement)
            .await?;

        let _ = self.finish_transaction(transaction.transaction_id, TransactionStatus::Completed).await?;

        if let Some(installment_id) = query.installment_id {
            let _ = self
                .update_installment_status(installment_id, TransactionStatus::Completed)
                .await?;
        }

        Ok(settlement)
    }

    pub async fn list_settlements(&self) -> Result<Vec<Settlement>> {
        let settlements = self.settlement_repository.list_settlements().await?;

        Ok(settlements)
    }
}
