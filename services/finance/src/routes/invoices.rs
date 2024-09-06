use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};

use crate::{domains::errors::Result, handlers::Handler};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest("/invoices", Router::new().route("", get(list_invoices)))
}

async fn list_invoices(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let invoices = handler.list_invoices().await?;

    Ok(Json::from(invoices))
}
