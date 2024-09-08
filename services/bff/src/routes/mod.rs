pub mod invoices;

use axum::Router;

use crate::handlers::Handler;

pub(crate) fn configure_services() -> Router<Handler> {
    Router::new().nest("/bff", invoices::configure_routes())
}
