use std::{net::SocketAddr, sync::Arc};

use handlers::Handler;
use repositories::SqlxRepository;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

pub mod domains;
pub mod handlers;
pub mod repositories;
pub mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // let max_pool_conections = std::env::var("MAX_POOL_CONECTIONS")
    //     .expect("Could not fetch max pool connections.")
    //     .parse::<u32>()
    //     .unwrap();

    let conn_str = std::env::var("DATABASE_URL").expect("Could not fetch connection string.");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&conn_str)
        .await
        .expect("Couldn't connect to the database");

    let sqlx_repository = Arc::new(SqlxRepository::new(pool));

    let handler = Handler::new(sqlx_repository.clone(), sqlx_repository);

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port.to_string()));

    // TODO: remove layer when the CORS is solved also remove the tower lib
    let app = routes::configure_routes()
        .with_state(handler)
        .layer(CorsLayer::permissive());

    let tpc_listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(tpc_listener, app).await.unwrap();
}
