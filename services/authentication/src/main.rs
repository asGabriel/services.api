use std::sync::Arc;

use repositories::SqlxRepository;
use sqlx::{migrate::MigrateError, postgres::PgPoolOptions, Pool, Postgres};

mod repositories;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let conn_str =
        std::env::var("LEGALENTITY_DATABASE_URL").expect("Could not fetch connection string.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_str)
        .await
        .expect("Couldn't connect to the database");

    let tes = execute_migration(&pool).await;
    println!("{tes:?}");

    let sqlx_repository = Arc::new(SqlxRepository::new(pool));
}

async fn execute_migration(pool: &Pool<Postgres>) -> Result<(), MigrateError> {
    sqlx::migrate!().run(pool).await
}
