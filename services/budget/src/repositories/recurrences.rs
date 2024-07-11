use crate::domains::{
    errors::Result,
    recurrences::{Frequency, Recurrences},
    transactions::{Category, MovementType},
};

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait RecurrenceRepository {
    async fn list_recurrences(&self) -> Result<Vec<Recurrences>>;
}

#[async_trait::async_trait]
impl RecurrenceRepository for SqlxRepository {
    async fn list_recurrences(&self) -> Result<Vec<Recurrences>> {
        let recurrences = sqlx::query_as!(
            Recurrences,
            r#"
            SELECT
                recurrence_id,
                account_id,
                title,
                frequency as "frequency!: Frequency",
                is_active,
                category as "category: Category",
                start_date,
                value,
                movement_type as "movement_type!: MovementType",
                created_at, 
                updated_at, 
                deleted_at
            FROM recurrences
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(recurrences)
    }
}
