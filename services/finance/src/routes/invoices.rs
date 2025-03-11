use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use http_problems::Result;
use uuid::Uuid;

use crate::{domains::invoices::InvoicePayload, handlers::Handler};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/invoices",
        Router::new()
            .route("/", get(list_invoices))
            .route("/:id", get(get_invoice_by_id))
            .route("/", post(create_invoice))
            .route("/:id/entries", get(get_invoice_and_subinvoices_entries)),
    )
}

async fn list_invoices(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let invoices = handler.list_invoices().await?;

    Ok(Json::from(invoices))
}

async fn create_invoice(
    State(handler): State<Handler>,
    Json(payload): Json<InvoicePayload>,
) -> Result<impl IntoResponse> {
    let invoice = handler.create_invoice(payload).await?;

    Ok(Json::from(invoice))
}

// TODO: implements an route to search with params: (year+month) or (uuid)
async fn get_invoice_by_id(
    State(handler): State<Handler>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let invoice = handler.get_invoice_by_id(id).await?;

    Ok(Json::from(invoice))
}

async fn get_invoice_and_subinvoices_entries(
    State(handler): State<Handler>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let invoice_details = handler.get_invoice_and_subinvoices_entries(id).await?;

    Ok(Json(invoice_details))
}
