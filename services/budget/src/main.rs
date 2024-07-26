use std::{sync::Arc, time::Duration};
use tokio::time;

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

    let conn_str = std::env::var("DATABASE_URL").expect("Could not fetch connection string.");

    tokio::spawn(periodic_task());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_str)
        .await
        .expect("Couldn't connect to the database");

    let sqlx_repository = Arc::new(SqlxRepository::new(pool));
    let handler = Handler::new(
        sqlx_repository.clone(),
        sqlx_repository.clone(),
        sqlx_repository.clone(),
        sqlx_repository.clone(),
        sqlx_repository,
    );

    let app = routes::configure_routes()
        .with_state(handler)
        .layer(CorsLayer::permissive());

    let port = std::env::var("PORT").expect("Could not fetch port data.");
    let url = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn periodic_task() {
    loop {
        // Sua lógica aqui
        println!("Executando tarefa periódica...");

        // Intervalo de espera antes da próxima execução (por exemplo, 1 segundo)
        time::sleep(Duration::from_secs(1)).await;
    }
}

#[macro_export]
macro_rules! update_fields {
    ($self:ident, $data:ident, $( $field:ident ),*) => {
        $(
            if let Some(value) = $data.$field {
                $self.$field = value;
            }
        )*
    };
}
