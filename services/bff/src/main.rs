use sqlx::postgres::PgPoolOptions;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let conn_str = std::env::var("DATABASE_URL").expect("Could not fetch connection string.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_str)
        .await
        .expect("Couldn't connect to the database");

    
}
