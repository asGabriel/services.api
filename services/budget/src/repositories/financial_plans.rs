use mockall::automock;

use crate::domains::{
    errors::Result,
    financial_plans::{FinancialPlan, MonthReference},
};

use super::SqlxRepository;

#[automock]
#[async_trait::async_trait]
pub trait FinancialPlanRepository {
    async fn create_financial_plan(&self, payload: FinancialPlan) -> Result<FinancialPlan>;
}

#[async_trait::async_trait]
impl FinancialPlanRepository for SqlxRepository {
    async fn create_financial_plan(&self, payload: FinancialPlan) -> Result<FinancialPlan> {
        let financial_plan = sqlx::query_as!(
            FinancialPlan,
            r#"
                INSERT INTO financial_plans 
                    (financial_plan_id, title, month, year, created_at)
                VALUES ($1, $2, $3, $4, $5) 
                RETURNING
                    financial_plan_id,
                    title,
                    month as "month: MonthReference",
                    year,
                    created_at, 
                    updated_at, 
                    deleted_at
            "#,
            payload.financial_plan_id,
            payload.title,
            payload.month as MonthReference,
            payload.year,
            payload.created_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(financial_plan)
    }
}
