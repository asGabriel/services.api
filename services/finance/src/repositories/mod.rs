use sqlx::PgPool;
pub mod invoices;

#[derive(Clone)]
pub struct SqlxRepository {
    pool: PgPool,
}

impl SqlxRepository {
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}