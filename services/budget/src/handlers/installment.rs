use crate::domains::{
    errors::Result,
    installments::{CreateInstallment, Installment},
};

use super::Handler;

impl Handler {
    pub async fn create_installment(
        &self,
        payload: CreateInstallment,
        steps: i16,
    ) -> Result<Vec<Installment>> {
        let mut installments: Vec<Installment> = Vec::new();
        dbg!(&steps);
        for step in 1..=steps {
            dbg!(&step);
            let result = self
                .installment_repository
                .create_installment(&payload, step)
                .await?;
            installments.push(result);
        }

        Ok(installments)
    }
}
