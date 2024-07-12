use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, patch, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    domains::{
        errors::Result,
        recurrences::{CreateRecurrence, UpdateRecurrence},
    },
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/recurrences",
        Router::new()
            .route("/", get(list_recurrences))
            .route("/", post(create_recurrence))
            .route("/:recurrence_id", get(get_recurrence_by_id))
            .route("/:recurrence_id", patch(update_recurrence_by_id)),
    )
}

async fn list_recurrences(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let recurrences = handler.list_recurrences().await?;

    Ok(Json::from(recurrences))
}

async fn create_recurrence(
    State(handler): State<Handler>,
    Json(payload): Json<CreateRecurrence>,
) -> Result<impl IntoResponse> {
    let recurrence = handler.create_recurrence(payload).await?;

    Ok(Json::from(recurrence))
}

async fn get_recurrence_by_id(
    State(handler): State<Handler>,
    Path(recurrence_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let recurrence = handler.get_recurrence_by_id(recurrence_id).await?;

    Ok(Json::from(recurrence))
}

async fn update_recurrence_by_id(
    State(handler): State<Handler>,
    Path(recurrence_id): Path<Uuid>,
    Json(payload): Json<UpdateRecurrence>,
) -> Result<impl IntoResponse> {
    let recurrence = handler.update_recurrence(recurrence_id, payload).await?;

    Ok(Json::from(recurrence))
}
