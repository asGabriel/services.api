use std::collections::BTreeMap;

use chrono::NaiveDate;
use mockall::automock;
use uuid::Uuid;

use crate::domains::{
    errors::Result,
    financial_plans::{FinancialPlan, MonthReference},
};

use super::SqlxRepository;

#[automock]
#[async_trait::async_trait]
pub trait FinancialPlanRepository {
    async fn create_financial_plan(&self, payload: FinancialPlan) -> Result<FinancialPlan>;
    async fn list_financial_plans(&self) -> Result<BTreeMap<NaiveDate, FinancialPlan>>;
    async fn get_financial_plan_by_id(
        &self,
        financial_plan_id: Uuid,
    ) -> Result<Option<FinancialPlan>>;
}

#[async_trait::async_trait]
impl FinancialPlanRepository for SqlxRepository {
    async fn list_financial_plans(&self) -> Result<BTreeMap<NaiveDate, FinancialPlan>> {
        let result = sqlx::query_as!(
            FinancialPlan,
            r#"
                SELECT
                    financial_plan_id,
                    title,
                    month as "month!: MonthReference",
                    year,
                    created_at,
                    updated_at,
                    deleted_at
                FROM financial_plans
            "#
        )
        .fetch_all(&self.pool)
        .await?;

    let mut financial_plans: BTreeMap<NaiveDate, FinancialPlan> = BTreeMap::new();
    for financial_plan in result {
        let reference_date = NaiveDate::from_ymd_opt(financial_plan.year as i32, financial_plan.month as u32, 01).unwrap();

        financial_plans.insert(reference_date, financial_plan);
    }

        Ok(financial_plans)
    }

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

    async fn get_financial_plan_by_id(
        &self,
        financial_plan_id: Uuid,
    ) -> Result<Option<FinancialPlan>> {
        let financial_plan = sqlx::query_as!(
            FinancialPlan,
            r#"
                SELECT
                    financial_plan_id,
                    title,
                    month as "month!: MonthReference",
                    year,
                    created_at,
                    updated_at,
                    deleted_at
                FROM financial_plans
                WHERE financial_plan_id = $1
            "#,
            financial_plan_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(financial_plan)
    }
}
