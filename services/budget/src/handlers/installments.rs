use chrono::{Datelike, NaiveDate};

use crate::domains::{
    errors::Result,
    installments::{CreateInstallment, Installment},
};

use super::Handler;

impl Handler {
    pub async fn create_installment(
        &self,
        mut payload: CreateInstallment,
        steps: i16,
    ) -> Result<Vec<Installment>> {
        let mut installments: Vec<Installment> = Vec::new();
        let mut reference_date = NaiveDate::from_ymd_opt(
            payload.due_date.year(),
            payload.due_date.month0(),
            payload.due_date.day0(),
        )
        .unwrap();

        for step in 1..=steps {
            let result = self
            .installment_repository
            .create_installment(&payload, step)
            .await?;

            reference_date = payload.next_due_date_by_frequency(reference_date);
            payload.due_date = reference_date;

            installments.push(result);
        }

        Ok(installments)
    }
}
