use std::{net::SocketAddr, sync::Arc};

use handlers::Handler;
use repositories::SqlxRepository;
use sqlx::postgres::PgPoolOptions;
use tokio_cron_scheduler::{Job, JobScheduler};
use tower_http::cors::CorsLayer;

pub mod domains;
pub mod handlers;
pub mod repositories;
pub mod routes;

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

    let handler = Handler::new(
        sqlx_repository.clone(),
        sqlx_repository.clone(),
        sqlx_repository.clone(),
        sqlx_repository.clone(),
        sqlx_repository,
    );

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "5010".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    start_scheduler(handler.clone().into()).await;

    // TODO: remove layer when the CORS is solved also remove the tower lib
    let app = routes::configure_routes()
        .with_state(handler)
        .layer(CorsLayer::permissive());

    let tpc_listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(tpc_listener, app).await.unwrap();
}

pub async fn start_scheduler(handler: Arc<Handler>) {
    let sched = JobScheduler::new().await.unwrap();

    let start_execution = handler.create_monthly_main_invoice().await;
    match start_execution {
        Ok(_) => {
            println!("Start invoice execution executed at {:?}", chrono::Utc::now());
        }
        Err(e) => eprintln!("Failed to start invoice execution job: {:?}", e),
    }

    let job_result = Job::new("0 0 12 * * * *", move |_uuid, _l| {
        let handler = Arc::clone(&handler);
        tokio::spawn(async move {
            println!("Job executed at {:?}", chrono::Utc::now());
            if let Err(e) = handler.create_monthly_main_invoice().await {
                eprintln!("Failed to create main invoice: {:?}", e);
            }
        });
    });

    match job_result {
        Ok(job) => {
            sched.add(job).await.unwrap();
            sched.start().await.unwrap();
        }
        Err(e) => eprintln!("Failed to create the cron job: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use tokio_cron_scheduler::{Job, JobScheduler};

    #[tokio::test]
    async fn test_valid_cron_expression() {
        let sched = JobScheduler::new().await.unwrap();

        let cron_expression = "0 0 * * * * *";

        let job = Job::new(cron_expression, |_uuid, _l| {
            println!("Test job executed!");
        });

        assert!(job.is_ok(), "Cron expression should be valid");

        if let Ok(valid_job) = job {
            let result = sched.add(valid_job).await;
            assert!(result.is_ok(), "The job should be added successfully");
        }
    }

    #[tokio::test]
    async fn test_invalid_cron_expression() {
        let cron_expression = "invalid cron";

        let job = Job::new(cron_expression, |_uuid, _l| {
            println!("This job should not run");
        });

        assert!(
            job.is_err(),
            "Invalid cron expression should return an error"
        );
    }
}
