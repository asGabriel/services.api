use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::{
    domains::{errors::Result, transactions::CreateTransactionDto},
    handlers::Handler,
};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/transactions",
        Router::new()
            .route("/", post(create_transaction))
            .route("/", get(list_transactions)),
    )
}

async fn create_transaction(
    State(handler): State<Handler>,
    Json(transaction): Json<CreateTransactionDto>,
) -> Result<impl IntoResponse> {
    let transaction = handler.create_transaction(transaction).await?;

    Ok(Json(transaction))
}

async fn list_transactions(State(handler): State<Handler>) -> Result<impl IntoResponse> {
    let transactions = handler.list_transactions().await?;

    Ok(Json(transactions))
}
