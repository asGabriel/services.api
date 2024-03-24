use axum::{routing::post, Router};

use crate::handlers::Handler;

pub(super) fn configure_routes() -> Router<Handler> {
    Router::new().nest(
        "/work-note",
        Router::new()
            .route("/", post(create_work_note))
    )
}

async fn create_work_note() {
    print!("Hi")
}
