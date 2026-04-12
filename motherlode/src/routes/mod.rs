use std::sync::Arc;
use sea_orm::DatabaseConnection;
use redis::aio::ConnectionManager;
use crate::{config::Config, payments::PaymentProvider};

pub mod auth;
pub mod users;
pub mod stokvels;
pub mod contributions;
pub mod wallet;
pub mod market;
pub mod discover;
pub mod fair_score;
pub mod investments;
pub mod messages;
pub mod notifications;
pub mod fraud;
pub mod webhooks;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: ConnectionManager,
    pub config: Config,
    pub payment_provider: Arc<dyn PaymentProvider>,
}
