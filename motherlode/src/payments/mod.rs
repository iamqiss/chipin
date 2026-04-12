//! Payment provider abstraction.
//! Swap between Ukheshe and Internal by changing PAYMENT_PROVIDER env var.
//! When FSP license is obtained, set PAYMENT_PROVIDER=internal.

use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use crate::config::Config;
use crate::errors::AppResult;

pub mod ukheshe;
pub mod internal;
pub mod types;

pub use types::*;

#[async_trait]
pub trait PaymentProvider: Send + Sync {
    /// Initiate a deposit into a user's wallet
    async fn deposit(&self, req: DepositRequest) -> AppResult<PaymentResponse>;

    /// Initiate a withdrawal from a user's wallet to their bank
    async fn withdraw(&self, req: WithdrawRequest) -> AppResult<PaymentResponse>;

    /// Transfer between two internal wallets
    async fn transfer(&self, req: TransferRequest) -> AppResult<PaymentResponse>;

    /// Check the status of a transaction
    async fn transaction_status(&self, provider_ref: &str) -> AppResult<TransactionStatus>;

    /// Verify a webhook payload signature
    fn verify_webhook(&self, payload: &[u8], signature: &str) -> AppResult<()>;

    /// Provider name for logging
    fn name(&self) -> &'static str;
}

/// Build the active payment provider from config
pub fn build_provider(config: &Config) -> anyhow::Result<std::sync::Arc<dyn PaymentProvider>> {
    match config.payment_provider.as_str() {
        "ukheshe" => {
            tracing::info!("Payment provider: Ukheshe");
            Ok(std::sync::Arc::new(ukheshe::UkhesheProvider::new(config)?))
        }
        "internal" => {
            tracing::info!("Payment provider: Internal (FSP)");
            Ok(std::sync::Arc::new(internal::InternalProvider::new(config)?))
        }
        other => anyhow::bail!("Unknown payment provider: {}", other),
    }
}
