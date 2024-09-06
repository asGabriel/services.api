pub mod invoices;

use axum::{http::StatusCode, response::IntoResponse, Router};

use crate::{domains::errors::Error, handlers::Handler};

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().merge(invoices::configure_routes())
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
        }
        .into_response()
    }
}
