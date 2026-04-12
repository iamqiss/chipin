//! Internal payment provider — activated post-FSP license.
//! Replaces Ukheshe with direct bank rails.

use async_trait::async_trait;
use crate::{config::Config, errors::AppResult, payments::*};

pub struct InternalProvider {
    // TODO: add internal payment rail config fields
}

impl InternalProvider {
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        if !config.internal_payments_enabled {
            anyhow::bail!("Internal payments not enabled. Set INTERNAL_PAYMENTS_ENABLED=true and ensure FSP license is active.");
        }
        Ok(Self {})
    }
}

#[async_trait]
impl PaymentProvider for InternalProvider {
    async fn deposit(&self, req: DepositRequest) -> AppResult<PaymentResponse> {
        todo!("Internal deposit — implement after FSP licensing")
    }

    async fn withdraw(&self, req: WithdrawRequest) -> AppResult<PaymentResponse> {
        todo!("Internal withdrawal — implement after FSP licensing")
    }

    async fn transfer(&self, req: TransferRequest) -> AppResult<PaymentResponse> {
        todo!("Internal transfer — implement after FSP licensing")
    }

    async fn transaction_status(&self, provider_ref: &str) -> AppResult<TransactionStatus> {
        todo!("Internal status — implement after FSP licensing")
    }

    fn verify_webhook(&self, payload: &[u8], signature: &str) -> AppResult<()> {
        todo!("Internal webhook — implement after FSP licensing")
    }

    fn name(&self) -> &'static str { "internal" }
}
