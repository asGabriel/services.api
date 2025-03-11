use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{delete, get, post}, Json, Router
};
use http_problems::Result;
use uuid::Uuid;

use crate::{domains::entries::EntryPayload, handlers::Handler};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/entries",
        Router::new()
            .route("/", get(list_entries))
            .route("/:id", get(get_entry_by_id))
            .route("/", post(create_entry))
            .route("/:id", delete(delete_entry_by_id)),
    )
}

async fn delete_entry_by_id(
    State(handler): State<Handler>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let _entry = handler.delete_entry_by_id(id).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn list_entries(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let entries = handler.list_entries().await?;

    Ok(Json::from(entries))
}

async fn get_entry_by_id(
    State(handler): State<Handler>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let entry = handler.get_entry_by_id(id).await?;

    Ok(Json::from(entry))
}

async fn create_entry(
    State(handler): State<Handler>,
    Json(payload): Json<EntryPayload>,
) -> Result<impl IntoResponse> {
    let entry = handler.create_entry(payload).await?;

    Ok(Json::from(entry))
}
