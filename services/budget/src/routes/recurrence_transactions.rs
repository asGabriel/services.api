use axum::{extract::State, response::IntoResponse, routing::{get, post}, Json, Router};

use crate::{
    domains::{errors::Result, recurrence_transactions::CreateRecurrenceTransaction},
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/recurrence_transactions",
        Router::new().route("/", post(create_recurrence_transaction)).route("/", get(list_recurrence_transactions))
    )
}

async fn create_recurrence_transaction(
    State(handler): State<Handler>,
    Json(payload): Json<CreateRecurrenceTransaction>,
) -> Result<impl IntoResponse> {
    let result = handler.create_recurrence_transaction(payload).await?;

    Ok(Json::from(result))
}

async fn list_recurrence_transactions(
    State(handler): State<Handler>
) -> Result<impl IntoResponse> {
    let result = handler.list_recurrence_transactions().await?;

    Ok(Json::from(result))
}
    