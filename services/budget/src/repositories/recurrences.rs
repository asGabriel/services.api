use crate::domains::{
    errors::Result,
    recurrences::{Frequency, Recurrence},
    transactions::{Category, MovementType},
};

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait RecurrenceRepository {
    async fn list_recurrences(&self) -> Result<Vec<Recurrence>>;
    async fn create_recurrence(&self, payload: Recurrence) -> Result<Recurrence>;
}

#[async_trait::async_trait]
impl RecurrenceRepository for SqlxRepository {
    async fn list_recurrences(&self) -> Result<Vec<Recurrence>> {
        let recurrences = sqlx::query_as!(
            Recurrence,
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

    async fn create_recurrence(&self, recurrence: Recurrence) -> Result<Recurrence> {
        let recurrence = sqlx::query_as!(
            Recurrence,
            r#"
            INSERT INTO recurrences(
                recurrence_id, 
                account_id, 
                title, 
                frequency, 
                is_active, 
                category, 
                start_date, 
                value, 
                movement_type
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING 
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
            "#,
            recurrence.recurrence_id,
            recurrence.account_id,
            recurrence.title,
            recurrence.frequency as Frequency,
            recurrence.is_active,
            recurrence.category as Category,
            recurrence.start_date,
            recurrence.value,
            recurrence.movement_type as MovementType
        ).fetch_one(&self.pool).await?;

        Ok(recurrence)
    }
}
