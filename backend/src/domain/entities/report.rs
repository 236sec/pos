use crate::domain::primitives::Money;

#[derive(Debug, Clone)]
pub struct DailySummary {
    pub date: chrono::NaiveDate,
    pub total_revenue: Money,
}
