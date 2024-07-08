use chrono::Months;
use finance::{
    installment::{Installment, InstallmentParams, PartialInstallment},
    transaction::Transaction,
};
use uuid::Uuid;

use super::Handler;
use crate::domains::errors::{Error, Result};

impl Handler {
    pub async fn create_installment(
        &self,
        transaction: &Transaction,
        installments: i16,
    ) -> Result<()> {
        let mut due_date = transaction.due_date;
        let mut params = InstallmentParams::new(0, installments);

        for step in 1..=installments {
            params.installment_number = step;
            due_date = due_date.checked_add_months(Months::new(1)).unwrap();

            let mut partial_installment = PartialInstallment::from_payload(&transaction, &params);

            partial_installment.due_date = due_date;

            let _ = self
                .installment_repository
                .create_installment(&partial_installment)
                .await?;
        }

        Ok(())
    }

    pub async fn get_installment_by_id(&self, installment_id: Uuid) -> Result<Installment> {
        self.installment_repository
            .get_installment_by_id(installment_id)
            .await?
            .ok_or(Error::InstallmentNotFound(installment_id))
    }
}
