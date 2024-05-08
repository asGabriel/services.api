use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    domains::{
        errors::Result,
        transactions::{CreateTransaction, TransactionStatus, UpdateTransaction},
    },
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/transactions",
        Router::new()
            .route("/", post(create_transaction))
            .route("/", get(list_transactions))
            .route("/:transaction_id", get(get_transaction_by_id))
            .route("/:transaction_id", delete(delete_transaction_by_id))
            .route("/:transaction_id", patch(update_transaction_by_id))
            .route("/:transaction_id/:status", post(finish_transaction)),
    )
}

async fn create_transaction(
    State(handler): State<Handler>,
    Json(transaction): Json<CreateTransaction>,
) -> Result<impl IntoResponse> {
    let transaction = handler.create_transaction(transaction).await?;

    Ok(Json(transaction))
}

async fn list_transactions(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let transactions = handler.list_transactions().await?;

    Ok(Json(transactions))
}

async fn get_transaction_by_id(
    State(handler): State<Handler>,
    Path(transaction_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let transaction = handler.get_transaction_by_id(transaction_id).await?;

    Ok(Json(transaction))
}

async fn delete_transaction_by_id(
    State(handler): State<Handler>,
    Path(transaction_id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    let transaction = handler.delete_transaction_by_id(transaction_id).await?;

    Ok(Json(transaction))
}

async fn update_transaction_by_id(
    State(handler): State<Handler>,
    Path(transaction_id): Path<Uuid>,
    Json(payload): Json<UpdateTransaction>,
) -> Result<impl IntoResponse> {
    let transaction = handler
        .update_transaction_by_id(transaction_id, payload)
        .await?;

    Ok(Json(transaction))
}

async fn finish_transaction(
    State(handler): State<Handler>,
    Path((transaction_id, status)): Path<(Uuid, TransactionStatus)>,
) -> Result<impl IntoResponse> {
    let transaction = handler.finish_transaction(transaction_id, status).await?;

    Ok(Json(transaction))
}
