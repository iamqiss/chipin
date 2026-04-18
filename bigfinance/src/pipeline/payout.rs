//! Payout pipeline — rotating stokvel payout to the next recipient.
//! Withdrawal fee applies. FX spread applies.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::Utc;
use uuid::Uuid;
use crate::currency::currencies::SupportedCurrency;
use crate::defi::aave::AaveClient;
use crate::fees::FeeCalculator;
use crate::currency::converter::CurrencyConverter;
use super::SettlementResult;

pub struct PayoutPipeline {
    pub converter: CurrencyConverter,
    pub fees:      FeeCalculator,
    pub aave:      AaveClient,
}

pub struct PayoutRequest {
    pub pool_id:          String,
    pub recipient_id:     String,
    pub usdc_amount:      Decimal,
    pub local_currency:   SupportedCurrency,
}

impl PayoutPipeline {
    pub async fn process(&mut self, req: PayoutRequest) -> anyhow::Result<SettlementResult> {
        let reference = format!("PAYOUT-{}", Uuid::new_v4());

        // Withdraw from Aave
        let withdrawn = self.aave.withdraw(req.usdc_amount).await?;

        // Apply payout fee (same as withdrawal)
        let fees     = self.fees.payout(withdrawn);
        let net_usdc = fees.net;

        // Convert to local currency
        let conversion = self.converter
            .usdc_to_local(net_usdc, req.local_currency).await?;

        let chipin_revenue = fees.withdrawal_fee + conversion.spread_revenue.amount;

        tracing::info!(
            reference    = %reference,
            recipient_id = %req.recipient_id,
            local_amount = %conversion.output.format(),
            chipin_rev   = %chipin_revenue,
            "Payout settled"
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
