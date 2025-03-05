use http_problems::Result;
use uuid::Uuid;

use crate::domains::entries::{Entry, EntryStatus, EntryType};

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait EntriesRepository {
    async fn list_entries(&self) -> Result<Vec<Entry>>;
    async fn get_entry_by_id(&self, entry_id: Uuid) -> Result<Option<Entry>>;
    async fn create_entry(&self, entry: Entry) -> Result<Entry>;
    async fn get_entries_by_invoice_id(&self, invoice_id: Uuid) -> Result<Vec<Entry>>;
    async fn delete_entry_by_id(&self, entry_id: Uuid) -> Result<()>;
}

#[async_trait::async_trait]
impl EntriesRepository for SqlxRepository {
    async fn delete_entry_by_id(&self, entry_id: Uuid) -> Result<()> {
        let _entry = sqlx::query!(
            r#"
                UPDATE entries
                SET
                    deleted_at = now()
                WHERE
                    entry_id = $1
                    
            "#,
            entry_id
        ).execute(&self.pool).await?;

        Ok(())
    }

    async fn get_entries_by_invoice_id(&self, invoice_id: Uuid) -> Result<Vec<Entry>> {
        let entries = sqlx::query_as!(
            Entry,
            r#"
                SELECT 
                    entry_id,
                    invoice_id,
                    entry_type as "entry_type!: EntryType",
                    description, 
                    value,
                    due_date,
                    account_id,
                    status as "status!: EntryStatus",
                    created_at,
                    updated_at,
                    deleted_at
                FROM entries
                WHERE
                    invoice_id = $1
                    AND deleted_at is null
            "#,
            invoice_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(entries)
    }

    async fn list_entries(&self) -> Result<Vec<Entry>> {
        let entries = sqlx::query_as!(
            Entry,
            r#"
                SELECT
                    entry_id,
                    invoice_id,
                    entry_type as "entry_type!: EntryType",
                    description, 
                    value,
                    due_date,
                    account_id,
                    status as "status!: EntryStatus",
                    created_at,
                    updated_at,
                    deleted_at
                FROM entries
                WHERE
                    deleted_at is null
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(entries)
    }

    async fn get_entry_by_id(&self, entry_id: Uuid) -> Result<Option<Entry>> {
        let entry = sqlx::query_as!(
            Entry,
            r#"
                SELECT
                    entry_id,
                    invoice_id,
                    entry_type as "entry_type!: EntryType",
                    description, 
                    value,
                    due_date,
                    account_id,
                    status as "status!: EntryStatus",
                    created_at,
                    updated_at,
                    deleted_at
                FROM entries
                WHERE
                    entry_id = $1
                    AND deleted_at is null
            "#,
            entry_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(entry)
    }

    async fn create_entry(&self, entry: Entry) -> Result<Entry> {
        let entry = sqlx::query_as!(
            Entry,
            r#"
                INSERT INTO entries (
                        entry_id,
                        invoice_id,
                        entry_type,
                        description, 
                        value,
                        due_date,
                        account_id,
                        status,
                        created_at,
                        updated_at,
                        deleted_at
                    ) VALUES (
                        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
                    ) RETURNING
                        entry_id,
                        invoice_id,
                        entry_type as "entry_type!: EntryType",
                        description, 
                        value,
                        due_date,
                        account_id,
                        status as "status!: EntryStatus",
                        created_at,
                        updated_at,
                        deleted_at
                
            "#,
            entry.entry_id,
            entry.invoice_id,
            entry.entry_type as EntryType,
            entry.description,
            entry.value.normalized(),
            entry.due_date,
            entry.account_id,
            entry.status as EntryStatus,
            entry.created_at,
            entry.updated_at,
            entry.deleted_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(entry)
    }
}
