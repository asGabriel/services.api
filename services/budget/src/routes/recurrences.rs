use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};

use crate::{domains::errors::Result, handlers::Handler};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest("/recurrences", Router::new().route("/", get(list_recurrences)))
}

async fn list_recurrences(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let recurrences = handler.list_recurrences().await?;

    Ok(Json::from(recurrences))
}