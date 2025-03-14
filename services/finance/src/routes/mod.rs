pub mod accounts;
pub mod entries;
pub mod invoices;
pub mod tags;

use axum::Router;

use crate::handlers::Handler;

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/finance",
        Router::new()
            .merge(invoices::configure_routes())
            .merge(entries::configure_routes())
            .merge(accounts::configure_routes())
            .merge(tags::configure_routes()),
    )
}
