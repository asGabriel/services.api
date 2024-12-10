use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use http_problems::Result;

use crate::handlers::Handler;

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest("/tags", Router::new().route("/", get(list_tags)))
}

async fn list_tags(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let tags = handler.list_tags().await?;

    Ok(Json(tags))
}
