use std::sync::Arc;

use repositories::SqlxRepository;
use sqlx::{migrate::MigrateError, postgres::PgPoolOptions, Pool, Postgres};

pub mod repositories;
pub mod handlers;
pub mod domains;
pub mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let conn_str =
        std::env::var("IAM_DATABASE_URL").expect("missing IAM_DATABASE_URL.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_str)
        .await
        .expect("Couldn't connect to the database");

    // execute_migration(&pool).await.expect("error running migrations");

    // let sqlx_repository = Arc::new(SqlxRepository::new(pool));

}

// async fn execute_migration(pool: &Pool<Postgres>) -> Result<(), MigrateError> {
//     sqlx::migrate!().run(pool).await
// }
