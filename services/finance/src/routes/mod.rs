pub mod invoices;
pub mod entries;

use axum::{http::StatusCode, response::IntoResponse, Router};

use crate::{domains::errors::Error, handlers::Handler};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().merge(invoices::configure_routes().merge(entries::configure_routes()))
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
            Self::InvoiceNotFound(id) => (StatusCode::NOT_FOUND, format!("Invoice id {id:?} not found")),
            Self::EntryNotFound(id) => (StatusCode::NOT_FOUND, format!("Entry id {id:?} not found")),
        }
        .into_response()
    }
}
