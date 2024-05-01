use uuid::Uuid;

use crate::domain::{
    errors::{Error, Result},
    work_note::{CreateWorkNote, UpdateWorkNote, WorkNote},
};

use super::Handler;

impl Handler {
    pub async fn create_work_note(&self, work_note: CreateWorkNote) -> Result<WorkNote> {
        self.work_note_repository.create_work_note(work_note).await
    }

    pub async fn list_work_notes(&self) -> Result<Vec<WorkNote>> {
        self.work_note_repository.list_work_notes().await
    }

    pub async fn get_work_note_by_id(&self, work_note_id: Uuid) -> Result<WorkNote> {
        self.work_note_repository
            .get_work_note_by_id(work_note_id)
            .await?
            .ok_or(Error::WorkNoteNotFound(work_note_id))
    }

    pub async fn update_work_note_by_id(
        &self,
        work_note_id: Uuid,
        work_note: UpdateWorkNote,
    ) -> Result<WorkNote> {
        self.work_note_repository
            .update_work_note_by_id(work_note_id, work_note)
            .await?
            .ok_or(Error::WorkNoteNotFound(work_note_id))
    }
}
