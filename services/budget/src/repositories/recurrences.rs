use std::collections::BTreeMap;

use mockall::automock;
use uuid::Uuid;

use crate::domains::{
    errors::Result,
    recurrences::{CreateRecurrenceLink, Frequency, Recurrence, RecurrenceLink},
    transactions::{Category, MovementType},
};

use super::SqlxRepository;

#[automock]
#[async_trait::async_trait]
pub trait RecurrenceRepository {
    async fn list_recurrences(&self) -> Result<Vec<Recurrence>>;
    async fn create_recurrence(&self, payload: Recurrence) -> Result<Recurrence>;
    async fn get_recurrence_by_id(&self, recurrence_id: Uuid) -> Result<Option<Recurrence>>;
    async fn update_recurrence(&self, payload: Recurrence) -> Result<Option<Recurrence>>;
    async fn get_recurrence_link(
        &self,
        recurrence_id: Vec<Uuid>,
    ) -> Result<BTreeMap<Uuid, Vec<RecurrenceLink>>>;
    async fn create_recurrence_link(&self, payload: CreateRecurrenceLink) -> Result<()>;
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
            recurrence.value.normalized(),
            recurrence.movement_type as MovementType
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(recurrence)
    }

    async fn get_recurrence_by_id(&self, recurrence_id: Uuid) -> Result<Option<Recurrence>> {
        let result = sqlx::query_as!(
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
            FROM 
                recurrences
            WHERE
                recurrence_id = $1
            "#,
            recurrence_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn update_recurrence(&self, payload: Recurrence) -> Result<Option<Recurrence>> {
        let result = sqlx::query_as!(
            Recurrence,
            r#"
            UPDATE 
                recurrences
            SET
                account_id = $2,
                title = $3,
                frequency = $4,
                is_active = $5,
                category = $6,
                start_date = $7,
                value = $8,
                movement_type = $9,
                updated_at = $10
            WHERE
                recurrence_id = $1
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
            payload.recurrence_id,
            payload.account_id,
            payload.title,
            payload.frequency as Frequency,
            payload.is_active,
            payload.category as Category,
            payload.start_date,
            payload.value.normalized(),
            payload.movement_type as MovementType,
            payload.updated_at
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn get_recurrence_link(
        &self,
        recurrence_id: Vec<Uuid>,
    ) -> Result<BTreeMap<Uuid, Vec<RecurrenceLink>>> {
        let result = sqlx::query_as!(
            RecurrenceLink,
            r#"
            SELECT
                links.recurrence_id,
                links.transaction_id,
                tr.due_date
            FROM 
                transaction_recurrence_links links
            INNER JOIN transactions tr ON links.transaction_id = tr.transaction_id 
            WHERE
                links.recurrence_id = any($1::uuid[])
            "#,
            &recurrence_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut links: BTreeMap<Uuid, Vec<RecurrenceLink>> = BTreeMap::new();

        for r in result {
            links
                .entry(r.recurrence_id)
                .or_insert_with(Vec::new)
                .push(r)
        }

        Ok(links)
    }

    async fn create_recurrence_link(&self, payload: CreateRecurrenceLink) -> Result<()> {
        match sqlx::query!(
            r#"
            INSERT INTO transaction_recurrence_links(transaction_id, recurrence_id) VALUES ($1, $2) RETURNING *
            "#,
            payload.transaction_id,
            payload.recurrence_id
        )
        .fetch_one(&self.pool)
        .await 
        {
            Ok(record) => {
                println!("Registro criado com sucesso: {:?}", record);
                Ok(())
            },
            Err(e) => {
                eprintln!("Erro ao criar registro: {:?}", e);
                Err(e.into())
            }
        }
    }
}
