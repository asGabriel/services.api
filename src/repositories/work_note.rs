use crate::domain::{errors::Result, work_note::WorkNote};

use super::SqlxRepository;

#[async_trait::async_trait]
pub trait WorkNoteRepository {
    async fn create_work_note(&self) -> Result<WorkNote>;
}

#[async_trait::async_trait]
impl WorkNoteRepository for SqlxRepository {
    async fn create_work_note(&self) -> Result<WorkNote> {
        let work = WorkNote {
            work_note_id: "sadasko".to_string()
        };

        Ok(work)
    }
}