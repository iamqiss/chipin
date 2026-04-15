//! gRPC Wallet service — TODO: implement after REST service is complete.

use tonic::{Request, Response, Status};
use sqlx::PgPool;

pub struct WalletServiceImpl {
    db: PgPool,
}

impl WalletServiceImpl {
    pub fn new(db: PgPool, _redis: redis::aio::ConnectionManager) -> Self {
        Self { db }
    }
}
