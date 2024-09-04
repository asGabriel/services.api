use std::sync::Arc;

use handlers::Handler;
use repositories::SqlxRepository;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

mod domains;
mod handlers;
mod repositories;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let max_pool_conections = std::env::var("MAX_POOL_CONECTIONS")
        .expect("Could not fetch max pool connections.")
        .parse::<u32>()
        .unwrap();

    let conn_str = std::env::var("DATABASE_URL").expect("Could not fetch connection string.");
    let pool = PgPoolOptions::new()
        .max_connections(max_pool_conections)
        .connect(&conn_str)
        .await
        .expect("Couldn't connect to the database");

    
    let sqlx_repository = Arc::new(SqlxRepository::new(pool));
    
    let handler = Handler::new();
    
    let port = std::env::var("PORT").expect("Could not fetch port data.");
    let url = format!("0.0.0.0:{}", port);
    
    // TODO: remove layer when the CORS is solved also remove the tower lib
    let app = routes::configure_routes()
    .with_state(handler)
    .layer(CorsLayer::permissive());

    let tpc_listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(tpc_listener, app).await.unwrap();
}
