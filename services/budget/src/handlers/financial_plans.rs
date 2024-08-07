use crate::domains::{
    errors::Result,
    financial_plans::{CreateFinancialPlan, FinancialPlan},
};

use super::Handler;

impl Handler {
    pub async fn create_financial_plan(
        &self,
        payload: CreateFinancialPlan,
    ) -> Result<FinancialPlan> {
        let payload = FinancialPlan::new_from_payload(payload);

        let financial_plan = self
            .financial_plan_repository
            .create_financial_plan(payload)
            .await?;

        Ok(financial_plan)
    }
}
