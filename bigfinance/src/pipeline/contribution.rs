//! Contribution pipeline.
//!
//! When a member contributes:
//!   1. Convert local → USDC
//!   2. Deduct platform fee (0.5%)
//!   3. Deposit net USDC into Aave
//!   4. Record revenue
//!   5. Return settlement confirmation

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::Utc;
use uuid::Uuid;
use crate::config::Config;
use crate::currency::{
    currencies::SupportedCurrency,
    converter::CurrencyConverter,
    rates::RateCache,
};
use crate::defi::aave::AaveClient;
use crate::fees::FeeCalculator;
use crate::revenue::RevenueRecorder;
use super::SettlementResult;

pub struct ContributionPipeline {
    pub converter: CurrencyConverter,
    pub fees:      FeeCalculator,
    pub aave:      AaveClient,
    pub config:    Config,
}

pub struct ContributionRequest {
    pub user_id:    String,
    pub pool_id:    String,
    pub amount:     Decimal,
    pub currency:   SupportedCurrency,
}

impl ContributionPipeline {
    pub async fn process(
        &mut self,
        req: ContributionRequest,
    ) -> anyhow::Result<SettlementResult> {
        let reference = format!("CONTRIB-{}", Uuid::new_v4());

        tracing::info!(
            reference = %reference,
            user_id   = %req.user_id,
            pool_id   = %req.pool_id,
            amount    = %req.amount,
            currency  = %req.currency.code(),
            "Processing contribution"
        );

        // Step 1: Convert local → USDC (includes FX spread)
        let conversion = self.converter
            .local_to_usdc(req.amount, req.currency.clone())
            .await?;

        let gross_usdc = conversion.output.amount;

        // Step 2: Apply platform fee
        let fee_result = self.fees.contribution(gross_usdc);

        let net_usdc = fee_result.net;

        // Step 3: Deposit net USDC into Aave
        let position = self.aave.deposit(net_usdc).await?;

        // Step 4: Calculate total chipin revenue
        let chipin_revenue = fee_result.chipin_revenue() + conversion.spread_revenue.amount;

        tracing::info!(
            reference     = %reference,
            gross_usdc    = %gross_usdc,
            net_usdc      = %net_usdc,
            platform_fee  = %fee_result.platform_fee,
            fx_spread     = %conversion.spread_revenue.amount,
            chipin_revenue = %chipin_revenue,
            apy           = %position.current_apy,
            "Contribution settled"
        );

        Ok(SettlementResult {
            reference,
            local_input:   crate::currency::currencies::Currency::new(req.amount, req.currency),
            usdc_settled:  net_usdc,
            chipin_revenue,
            partner_revenue: dec!(0),
            is_in_yield:   true,
            current_apy:   Some(position.current_apy),
            settled_at:    Utc::now().to_rfc3339(),
        })
    }
}
