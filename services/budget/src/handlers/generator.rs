use chrono::NaiveDate;
use log::info;

use crate::domains::{
    errors::Result,
    financial_plans::{CreateFinancialPlan, FinancialPlan},
    generator::GeneratorContext,
};

use super::Handler;

impl Handler {
    /// Generate a financial plan from a given date.
    pub async fn generate_financial_plan_from_date(
        &self,
        date: NaiveDate,
    ) -> Result<FinancialPlan> {
        let active_plans = self
            .financial_plan_repository
            .list_financial_plans()
            .await?;

        let context = GeneratorContext {
            financial_plans: active_plans,
        };

        if let Some(result) = context.get_current_financial_plan(date) {
            return Ok(result.clone());
        }

        let payload = CreateFinancialPlan::new(date);
        let financial_plan = self.create_financial_plan(payload).await?;
        info!("Creating new financial plan: {:?}", financial_plan);

        Ok(financial_plan)
    }

    /*
    criar um plano financeiro
    buscar as transações recorrentes do mês
    buscar as parcelas do mês
    */
}

/*
    implementar testes para o gerador de planos financeiros
    generate_financial_plan_from_date()
*/
