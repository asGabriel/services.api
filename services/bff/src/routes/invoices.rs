use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use http_problems::errors::Result;

use crate::handlers::Handler;

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest("/operations", Router::new().route("/", get(get_operations)))
}

async fn get_operations(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let invoices = handler.list_invoices().await?;

    Ok(Json::from(invoices))
}
