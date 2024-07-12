use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::{
    domains::{errors::Result, recurrences::CreateRecurrence},
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/recurrences",
        Router::new()
            .route("/", get(list_recurrences))
            .route("/", post(create_recurrence)),
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
