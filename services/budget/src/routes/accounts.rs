use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};

use crate::{domains::errors::Result, handlers::Handler};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest("/accounts", Router::new().route("/", get(list_accounts)))
}

async fn list_accounts(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let accounts = handler.list_accounts().await?;

    Ok(Json(accounts))
}
