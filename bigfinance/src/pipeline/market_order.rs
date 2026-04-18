//! Market order pipeline — bulk grocery order processing.
//! Platform fee + partner cut both apply.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::Utc;
use uuid::Uuid;
use crate::currency::currencies::SupportedCurrency;
use crate::currency::converter::CurrencyConverter;
use crate::fees::FeeCalculator;
use super::SettlementResult;

pub struct MarketOrderPipeline {
    pub converter: CurrencyConverter,
    pub fees:      FeeCalculator,
}

pub struct MarketOrderRequest {
    pub pool_id:     String,
    pub retailer_id: String,
    pub amount:      Decimal,
    pub currency:    SupportedCurrency,
}

impl MarketOrderPipeline {
    pub async fn process(&mut self, req: MarketOrderRequest) -> anyhow::Result<SettlementResult> {
        let reference = format!("MARKET-{}", Uuid::new_v4());

        // Convert to USDC
        let conversion = self.converter
            .local_to_usdc(req.amount, req.currency.clone()).await?;

        let gross_usdc = conversion.output.amount;

        // Apply market order fees (platform + partner)
        let fees = self.fees.market_order(gross_usdc);

        let chipin_revenue  = fees.platform_fee + conversion.spread_revenue.amount;
        let partner_revenue = fees.partner_cut;

        tracing::info!(
            reference      = %reference,
            retailer_id    = %req.retailer_id,
            gross_usdc     = %gross_usdc,
            chipin_rev     = %chipin_revenue,
            partner_rev    = %partner_revenue,
            "Market order settled"
        );

        Ok(SettlementResult {
            reference,
            local_input:    crate::currency::currencies::Currency::new(req.amount, req.currency),
            usdc_settled:   fees.net,
            chipin_revenue,
            partner_revenue,
            is_in_yield:    false,
            current_apy:    None,
            settled_at:     Utc::now().to_rfc3339(),
        })
    }
}
