//! Withdrawal pipeline.
//!
//! When a member withdraws:
//!   1. Withdraw USDC from Aave
//!   2. Deduct withdrawal fee (0.25%)
//!   3. Convert USDC → local (with FX spread)
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
};
use crate::defi::aave::AaveClient;
use crate::fees::FeeCalculator;
use super::SettlementResult;

pub struct WithdrawalPipeline {
    pub converter: CurrencyConverter,
    pub fees:      FeeCalculator,
    pub aave:      AaveClient,
}

pub struct WithdrawalRequest {
    pub user_id:         String,
    pub pool_id:         String,
    pub usdc_amount:     Decimal,
    pub target_currency: SupportedCurrency,
}

impl WithdrawalPipeline {
    pub async fn process(
        &mut self,
        req: WithdrawalRequest,
    ) -> anyhow::Result<SettlementResult> {
        let reference = format!("WITHDRAW-{}", Uuid::new_v4());

        // Step 1: Withdraw from Aave
        let usdc_withdrawn = self.aave.withdraw(req.usdc_amount).await?;

        // Step 2: Apply withdrawal fee
        let fee_result = self.fees.withdrawal(usdc_withdrawn);
        let net_usdc   = fee_result.net;

        // Step 3: Convert USDC → local (FX spread applied here)
        let conversion = self.converter
            .usdc_to_local(net_usdc, req.target_currency.clone())
            .await?;

        let chipin_revenue = fee_result.withdrawal_fee + conversion.spread_revenue.amount;

        tracing::info!(
            reference  = %reference,
            usdc_out   = %net_usdc,
            local_out  = %conversion.output.format(),
            chipin_rev = %chipin_revenue,
            "Withdrawal settled"
        );

        Ok(SettlementResult {
            reference,
            local_input:    conversion.output,
            usdc_settled:   net_usdc,
            chipin_revenue,
            partner_revenue: dec!(0),
            is_in_yield:    false,
            current_apy:    None,
            settled_at:     Utc::now().to_rfc3339(),
        })
    }
}
