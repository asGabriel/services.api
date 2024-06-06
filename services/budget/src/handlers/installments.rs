use crate::domains::{
    errors::Result,
    installments::{Installment, InstallmentParams},
    transactions::Transaction,
};

use super::Handler;

impl Handler {
    pub async fn create_installment(
        &self,
        transaction: &Transaction,
        params: InstallmentParams,
    ) -> Result<Vec<Installment>> {
        let mut installments: Vec<Installment> = Vec::new();
        let mut payload = transaction.generate_installment_payload();

        for step in 1..=transaction.installment_number {
            let result = self
                .installment_repository
                .create_installment(&payload, &params, step)
                .await?;

            payload.next_due_date_by_frequency();
            installments.push(result);
        }

        Ok(installments)
    }
}
