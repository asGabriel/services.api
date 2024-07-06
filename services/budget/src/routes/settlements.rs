use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::{
    domains::{errors::Result, settlements::CreateSettlement},
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/settlements",
        Router::new()
            .route("/", get(list_settlements))
            .route("/", post(create_settlement)),
    )
}

async fn list_settlements(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let settlements = handler.list_settlements().await?;

    Ok(Json::from(settlements))
}

async fn create_settlement(
    State(handler): State<Handler>,
    Json(payload): Json<CreateSettlement>,
) -> Result<impl IntoResponse> {
    let settlement = handler.create_settlement(payload).await?;

    Ok(Json::from(settlement))
}
