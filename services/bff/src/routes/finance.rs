use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use http_problems::errors::Result;
use uuid::Uuid;

use crate::handlers::Handler;

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/financial",
        Router::new()
            .route("/operations", get(get_operations))
            .route("/invoice/:id", get(get_operations_by_invoice_id)),
    )
}

async fn get_operations(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let operations = handler.get_operations().await?;

    Ok(Json::from(operations))
}

async fn get_operations_by_invoice_id(
    State(handler): State<Handler>,
    Path(invoice_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let invoice_details = handler.get_entries_by_invoice_id(invoice_id).await?;

    Ok(Json::from(invoice_details))
}
