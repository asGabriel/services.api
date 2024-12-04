use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use http_problems::Result;
use uuid::Uuid;

use crate::{domains::accounts::CreateAccountPayload, handlers::Handler};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/accounts",
        Router::new()
            .route("/", get(list_accounts))
            .route("/:id", get(get_account_by_id))
            .route("/", post(create_account)),
    )
}

async fn list_accounts(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let accounts = handler.list_accounts().await?;

    Ok(Json(accounts))
}

async fn get_account_by_id(
    State(handler): State<Handler>,
    Path(account_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let account = handler.get_account_by_id(account_id).await?;

    Ok(Json(account))
}

async fn create_account(
    State(handler): State<Handler>,
    Json(payload): Json<CreateAccountPayload>,
) -> Result<impl IntoResponse> {
    let account = handler.create_account(payload).await?;

    Ok(Json(account))
}
