pub mod entries;
pub mod invoices;

use axum::Router;

use crate::handlers::Handler;

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().merge(invoices::configure_routes().merge(entries::configure_routes()))
}
