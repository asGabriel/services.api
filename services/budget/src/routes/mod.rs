pub mod accounts;
pub mod settlements;
pub mod transactions;
pub mod views;

use axum::{http::StatusCode, response::IntoResponse, Router};

use crate::{domains::errors::Error, handlers::Handler};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new()
        .merge(transactions::configure_routes())
        .merge(accounts::configure_routes())
        .merge(views::configure_routes())
        .merge(settlements::configure_routes())
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
            Self::TransactionNotFound(transaction_id) => (
                StatusCode::NOT_FOUND,
                format!("Transaction id {transaction_id} not found."),
            ),
            Self::AccountNotFound(account_id) => (
                StatusCode::NOT_FOUND,
                format!("Account id {account_id} not found."),
            ),
            Self::AccountAlreadyDeleted(account_id) => (
                StatusCode::NOT_FOUND,
                format!("Account id {account_id} has been already deleted."),
            ),
            Self::TransactionFinished(transaction_id) => (
                StatusCode::BAD_REQUEST,
                format!("Transaction id {transaction_id} has been already finished."),
            ),
        }
        .into_response()
    }
}
