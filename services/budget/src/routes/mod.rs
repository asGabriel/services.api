pub mod transactions;

use axum::{http::StatusCode, response::IntoResponse, Router};

use crate::{domains::errors::Error, handlers::Handler};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().merge(transactions::configure_routes())
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
            Self::TransactionNotFound(transaction_id) => (
                StatusCode::NOT_FOUND,
                format!("Transaction id {transaction_id} not found."),
            ),
        }
        .into_response()
    }
}
