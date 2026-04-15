//! gRPC Stokvels service — TODO: implement after REST service is complete.

use tonic::{Request, Response, Status};
use sqlx::PgPool;

pub struct StokvelsServiceImpl {
    db: PgPool,
}

impl StokvelsServiceImpl {
    pub fn new(db: PgPool, _redis: redis::aio::ConnectionManager) -> Self {
        Self { db }
    }
}
