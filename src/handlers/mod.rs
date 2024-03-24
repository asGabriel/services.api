use std::sync::Arc;

use crate::repositories::work_note::WorkNoteRepository;

#[derive(Clone)]
pub struct Handler {
    work_note_repository: Arc<dyn WorkNoteRepository + Send + Sync>
}

impl Handler {
    pub const fn new(
        work_note_repository: Arc<dyn WorkNoteRepository + Send + Sync>
    ) -> Self {
        Self {
            work_note_repository
        }
    }
}