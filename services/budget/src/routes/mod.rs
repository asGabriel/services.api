pub mod accounts;
pub mod financial_plans;
pub mod recurrences;
pub mod settlements;
pub mod transactions;

use axum::{http::StatusCode, response::IntoResponse, Router};

use crate::{domains::errors::Error, handlers::Handler};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new()
        .merge(transactions::configure_routes())
        .merge(accounts::configure_routes())
        .merge(settlements::configure_routes())
        .merge(recurrences::configure_routes())
        .merge(financial_plans::configure_routes())
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
            Self::TransactionNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Transaction id {id} not found."),
            ),
            Self::AccountNotFound(id) => {
                (StatusCode::NOT_FOUND, format!("Account id {id} not found."))
            }
            Self::AccountAlreadyDeleted(id) => (
                StatusCode::NOT_FOUND,
                format!("Account id {id} has been already deleted."),
            ),
            Self::TransactionFinished(id) => (
                StatusCode::BAD_REQUEST,
                format!("Transaction id {id} has been already finished."),
            ),
            Self::InstallmentFinished(id) => (
                StatusCode::BAD_REQUEST,
                format!("Installment id {id} has been already finished."),
            ),
            Self::InstallmentNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Installment id {id} not found."),
            ),
            Self::RecurrenceNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Recurrence id {id} not found."),
            ),
        }
        .into_response()
    }
}
