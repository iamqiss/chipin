//! gRPC Fair_Score service — TODO: implement after REST service is complete.

use tonic::{Request, Response, Status};
use sqlx::PgPool;

pub struct FairScoreServiceImpl {
    db: PgPool,
}

impl FairScoreServiceImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}
