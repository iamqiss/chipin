//! gRPC Market service — TODO: implement after REST service is complete.

use tonic::{Request, Response, Status};
use sqlx::PgPool;

pub struct MarketServiceImpl {
    db: PgPool,
}

impl MarketServiceImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}
