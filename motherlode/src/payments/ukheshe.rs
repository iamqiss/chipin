//! Ukheshe payment provider implementation.
//! Active until FSP license is obtained — then swap to internal.

use async_trait::async_trait;
use crate::{config::Config, errors::{AppError, AppResult}, payments::*};

pub struct UkhesheProvider {
    base_url: String,
    client_id: String,
    client_secret: String,
    webhook_secret: String,
    http: reqwest::Client,
}

impl UkhesheProvider {
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self {
            base_url: config.ukheshe_base_url.clone(),
            client_id: config.ukheshe_client_id.clone(),
            client_secret: config.ukheshe_client_secret.clone(),
            webhook_secret: config.ukheshe_webhook_secret.clone(),
            http: reqwest::Client::new(),
        })
    }
}

#[async_trait]
impl PaymentProvider for UkhesheProvider {
    async fn deposit(&self, req: DepositRequest) -> AppResult<PaymentResponse> {
        // TODO: implement Ukheshe deposit API call
        todo!("Ukheshe deposit")
    }

    async fn withdraw(&self, req: WithdrawRequest) -> AppResult<PaymentResponse> {
        // TODO: implement Ukheshe withdrawal API call
        todo!("Ukheshe withdrawal")
    }

    async fn transfer(&self, req: TransferRequest) -> AppResult<PaymentResponse> {
        // TODO: implement Ukheshe transfer API call
        todo!("Ukheshe transfer")
    }

    async fn transaction_status(&self, provider_ref: &str) -> AppResult<TransactionStatus> {
        // TODO: implement Ukheshe status check
        todo!("Ukheshe status")
    }

    fn verify_webhook(&self, payload: &[u8], signature: &str) -> AppResult<()> {
        // TODO: HMAC verification against webhook_secret
        todo!("Ukheshe webhook verification")
    }

    fn name(&self) -> &'static str { "ukheshe" }
}
