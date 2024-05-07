use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    domains::{
        accounts::{CreateAccount, UpdateAccount},
        errors::Result,
    },
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/accounts",
        Router::new()
            .route("/", get(list_accounts))
            .route("/", post(create_account))
            .route("/:account_id", get(get_account_by_id))
            .route("/:account_id", delete(delete_account_by_id))
            .route("/:account_id", patch(update_account_by_id)),
    )
}

async fn list_accounts(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let accounts = handler.list_accounts().await?;

    Ok(Json(accounts))
}

async fn create_account(
    State(handler): State<Handler>,
    Json(payload): Json<CreateAccount>,
) -> Result<impl IntoResponse> {
    let account = handler.create_account(payload).await?;

    Ok(Json(account))
}

async fn get_account_by_id(
    State(handler): State<Handler>,
    Path(account_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let account = handler.get_account_by_id(account_id).await?;

    Ok(Json(account))
}

async fn delete_account_by_id(
    State(handler): State<Handler>,
    Path(account_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let account = handler.delete_account_by_id(account_id).await?;

    Ok(Json(account))
}

async fn update_account_by_id(
    State(handler): State<Handler>,
    Path(account_id): Path<Uuid>,
    Json(payload): Json<UpdateAccount>,
) -> Result<impl IntoResponse> {
    let account = handler.update_account_by_id(account_id, payload).await?;

    Ok(Json(account))
}
