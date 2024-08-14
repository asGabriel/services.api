use std::collections::BTreeMap;

use chrono::{Datelike, NaiveDate};

use super::financial_plans::FinancialPlan;

pub struct GeneratorContext {
    pub financial_plans: BTreeMap<NaiveDate, FinancialPlan>,
}

impl GeneratorContext {
    pub fn get_current_financial_plan(&self, due_date: NaiveDate) -> Option<&FinancialPlan> {
        let month = due_date.month();
        let year = due_date.year();

        self.financial_plans
            .iter()
            .find(|(date, _)| date.month() == month && date.year() == year)
            .map(|(_, plan)| plan)
    }
}
