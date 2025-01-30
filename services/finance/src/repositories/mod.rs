use sqlx::PgPool;
pub mod accounts;
pub mod entries;
pub mod invoice_relations;
pub mod invoices;
pub mod tags;

#[derive(Clone)]
pub struct SqlxRepository {
    pool: PgPool,
}

impl SqlxRepository {
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
