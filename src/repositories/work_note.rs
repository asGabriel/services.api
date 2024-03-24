use crate::domain::{
    errors::Result,
    work_note::{CreateWorkNote, WorkNote},
};

use super::SqlxRepository;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait WorkNoteRepository {
    async fn create_work_note(&self, work_note: CreateWorkNote) -> Result<WorkNote>;
    async fn list_work_notes(&self) -> Result<Vec<WorkNote>>;
    async fn get_work_note_by_id(&self, work_note_id: Uuid) -> Result<Option<WorkNote>>;
}

#[async_trait::async_trait]
impl WorkNoteRepository for SqlxRepository {
    async fn create_work_note(&self, work_note: CreateWorkNote) -> Result<WorkNote> {
        let work_note = sqlx::query_as!(
            WorkNote,
            r#"
            INSERT INTO WORKNOTES (
                WORK_NOTE_ID,
                CATEGORY,
                WORK_DATE,
                WORK_HOURS,
                OBSERVATION
            ) VALUES (
                $1, $2, $3, $4, $5
            ) RETURNING *
            "#,
            Uuid::new_v4(),
            work_note.category,
            work_note.work_date,
            work_note.work_hours,
            work_note.observation
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(work_note)
    }

    async fn list_work_notes(&self) -> Result<Vec<WorkNote>> {
        let work_notes = sqlx::query_as!(
            WorkNote,
            r#"
            SELECT * FROM WORKNOTES
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(work_notes)
    }

    async fn get_work_note_by_id(&self, work_note_id: Uuid) -> Result<Option<WorkNote>> {
        let work_note = sqlx::query_as!(
            WorkNote,
            r#"
            SELECT * FROM WORKNOTES WHERE WORK_NOTE_ID = $1
            "#,
            work_note_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(work_note)
    }
}
