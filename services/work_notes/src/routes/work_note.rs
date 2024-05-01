use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, patch, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    domain::{
        errors::Result,
        work_note::{CreateWorkNote, UpdateWorkNote},
    },
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/work-notes",
        Router::new()
            .route("/", post(create_work_note))
            .route("/", get(list_work_notes))
            .route("/:work_note_id", get(get_work_note_by_id))
            .route("/:work_note_id", patch(update_work_note_by_id)),
    )
}

async fn create_work_note(
    State(handler): State<Handler>,
    Json(work_note): Json<CreateWorkNote>,
) -> Result<impl IntoResponse> {
    let work_note = handler.create_work_note(work_note).await?;

    Ok(Json::from(work_note))
}

async fn list_work_notes(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let work_notes = handler.list_work_notes().await?;

    Ok(Json::from(work_notes))
}

async fn get_work_note_by_id(
    State(handler): State<Handler>,
    Path(work_note_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let work_note = handler.get_work_note_by_id(work_note_id).await?;

    Ok(Json::from(work_note))
}

async fn update_work_note_by_id(
    State(handler): State<Handler>,
    Path(work_note_id): Path<Uuid>,
    Json(work_note): Json<UpdateWorkNote>,
) -> Result<impl IntoResponse> {
    let work_note = handler
        .update_work_note_by_id(work_note_id, work_note)
        .await?;

    Ok(Json::from(work_note))
}
