use std::sync::Arc;

use handlers::Handler;
use repositories::SqlxRepository;
use sqlx::postgres::PgPoolOptions;
mod domain;
mod handlers;
mod repositories;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let conn_str = std::env::var("DATABASE_URL").expect("Could not fetch connection string.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_str)
        .await
        .expect("Couldn't connect to the database");

    let sqlx_repository = Arc::new(SqlxRepository::new(pool));
    let handler = Handler::new(sqlx_repository);

    let app = routes::configure_routes().with_state(handler);

    let port = std::env::var("PORT").expect("Could not fetch port data.");
    let url = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
