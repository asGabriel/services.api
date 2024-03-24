use axum::{http::StatusCode, response::IntoResponse, Router};
use crate::{domain::errors::Error, handlers::Handler};
pub mod work_note;


pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().merge(work_note::configure_routes())
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}")),
        }
        .into_response()
    }
}
