//! gRPC Fraud service — TODO: implement after REST service is complete.

use tonic::{Request, Response, Status};
use sqlx::PgPool;

pub struct FraudServiceImpl {
    db: PgPool,
}

impl FraudServiceImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}
