//! gRPC Messages service — TODO: implement after REST service is complete.

use tonic::{Request, Response, Status};
use sqlx::PgPool;

pub struct MessagesServiceImpl {
    db: PgPool,
}

impl MessagesServiceImpl {
    pub fn new(db: PgPool, _redis: redis::aio::ConnectionManager) -> Self {
        Self { db }
    }
}
