use crate::{domain::errors::Error, handlers::Handler};
use axum::{http::StatusCode, response::IntoResponse, Router};
pub mod work_note;

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().merge(work_note::configure_routes())
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
            Self::WorkNoteNotFound(work_note_id) => (
                StatusCode::NOT_FOUND,
                format!("Work note {work_note_id} not found"),
            ),
        }
        .into_response()
    }
}
