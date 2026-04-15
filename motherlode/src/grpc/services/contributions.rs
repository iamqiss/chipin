//! gRPC Contributions service — TODO: implement after REST service is complete.

use tonic::{Request, Response, Status};
use sqlx::PgPool;

pub struct ContributionsServiceImpl {
    db: PgPool,
}

impl ContributionsServiceImpl {
    pub fn new(db: PgPool, _redis: redis::aio::ConnectionManager) -> Self {
        Self { db }
    }
}
