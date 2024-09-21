use std::sync::Arc;

use repositories::SqlxRepository;
use sqlx::{migrate::MigrateError, postgres::PgPoolOptions, Pool, Postgres};

pub mod domains;
pub mod handlers;
pub mod repositories;
pub mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let conn_str = std::env::var("DATABASE_URL").expect("missing env DATABASE_URL.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_str)
        .await
        .expect("Couldn't connect to the database");

    // let sqlx_repository = Arc::new(SqlxRepository::new(pool));
}
