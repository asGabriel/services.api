use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};

use crate::domains::errors::Result;

use super::AppState;

pub(super) fn configure_router() -> Router<AppState> {
    Router::new().route("/budget/transactions", get(get_transactions))
}

async fn get_transactions(State(handler): State<AppState>) -> Result<impl IntoResponse> {
    let transactions = handler.budget_handler.get_transactions().await?;

    Ok(Json(transactions))
}
