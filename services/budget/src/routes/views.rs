use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};

use crate::{domains::errors::Result, handlers::Handler};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().route("/views/requestsPage", get(requests_page_view))
}

async fn requests_page_view(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    Ok(Json::from({}))
}
