use uuid::Uuid;

use crate::domains::{
    errors::{Error, Result},
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

    pub async fn list_financial_plans(&self) -> Result<Vec<FinancialPlan>> {
        let financial_plans = self
            .financial_plan_repository
            .list_financial_plans()
            .await?;

        Ok(financial_plans)
    }

    pub async fn get_financial_plan_by_id(&self, financial_plan_id: Uuid) -> Result<FinancialPlan> {
        self.financial_plan_repository
            .get_financial_plan_by_id(financial_plan_id)
            .await?
            .ok_or(Error::FinancialPlanNotFound(financial_plan_id))
    }
}
