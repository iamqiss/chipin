//! Currency conversion engine.
//!
//! chipin's conversion flow:
//!   local → USDC (settlement)
//!   USDC → local (payout)
//!
//! The FX spread (chipin's cut) is applied on every conversion.
//! Users see the net amount — the spread is invisible to them
//! but fully accounted for in revenue.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::currency::{
    currencies::{Currency, SupportedCurrency},
    rates::{ExchangeRate, RateCache, RateSource},
};
use crate::fees::FeeResult;
use crate::config::Config;

pub struct CurrencyConverter {
    pub rate_cache: RateCache,
    pub config:     Config,
}

/// Result of a currency conversion.
#[derive(Debug, Clone)]
pub struct ConversionResult {
    /// What the user started with
    pub input:           Currency,
    /// What they end up with (after spread)
    pub output:          Currency,
    /// The rate used
    pub rate:            Decimal,
    /// chipin's FX spread revenue
    pub spread_revenue:  Currency,
    /// Whether rate was live or cached
    pub rate_source:     RateSource,
}

impl CurrencyConverter {
    pub fn new(rate_cache: RateCache, config: Config) -> Self {
        Self { rate_cache, config }
    }

    /// Convert local currency to USDC for settlement.
    /// Applies FX spread — chipin keeps the spread as revenue.
    pub async fn local_to_usdc(
        &mut self,
        amount: Decimal,
        from: SupportedCurrency,
    ) -> anyhow::Result<ConversionResult> {
        if from == SupportedCurrency::USDC {
            return Ok(ConversionResult {
                input:          Currency::usdc(amount),
                output:         Currency::usdc(amount),
                rate:           dec!(1),
                spread_revenue: Currency::usdc(dec!(0)),
                rate_source:    RateSource::Hardcoded,
            });
        }

        let rate = self.get_rate(&from).await?;
        let spread = Config::bps_to_decimal(self.config.fx_spread_bps);

        // Gross USDC before spread
        let gross_usdc = amount / rate.usdc_to_local;

        // Spread revenue stays with chipin
        let spread_amount = gross_usdc * spread;

        // Net USDC user gets
        let net_usdc = gross_usdc - spread_amount;

        Ok(ConversionResult {
            input:          Currency::new(amount, from),
            output:         Currency::usdc(net_usdc),
            rate:           rate.usdc_to_local,
            spread_revenue: Currency::usdc(spread_amount),
            rate_source:    rate.source,
        })
    }

    /// Convert USDC to local currency for payout.
    /// Also applies FX spread on the way out.
    pub async fn usdc_to_local(
        &mut self,
        usdc_amount: Decimal,
        to: SupportedCurrency,
    ) -> anyhow::Result<ConversionResult> {
        if to == SupportedCurrency::USDC {
            return Ok(ConversionResult {
                input:          Currency::usdc(usdc_amount),
                output:         Currency::usdc(usdc_amount),
                rate:           dec!(1),
                spread_revenue: Currency::usdc(dec!(0)),
                rate_source:    RateSource::Hardcoded,
            });
        }

        let rate = self.get_rate(&to).await?;
        let spread = Config::bps_to_decimal(self.config.fx_spread_bps);

        // Gross local before spread
        let gross_local = usdc_amount * rate.usdc_to_local;

        // Spread stays with chipin (deducted from gross)
        let spread_usdc  = usdc_amount * spread;
        let spread_local = spread_usdc * rate.usdc_to_local;

        // Net local user receives
        let net_local = gross_local - spread_local;

        Ok(ConversionResult {
            input:          Currency::usdc(usdc_amount),
            output:         Currency::new(net_local, to),
            rate:           rate.usdc_to_local,
            spread_revenue: Currency::usdc(spread_usdc),
            rate_source:    rate.source,
        })
    }

    /// Get exchange rate — cache first, oracle fallback.
    async fn get_rate(&mut self, currency: &SupportedCurrency) -> anyhow::Result<ExchangeRate> {
        // Try cache first
        if let Some(cached) = self.rate_cache.get(currency).await {
            return Ok(cached);
        }

        // Cache miss — fetch from oracle
        let rate = crate::oracle::fetch_rate(currency, &self.config).await?;
        self.rate_cache.set(&rate).await.ok(); // non-critical if cache fails
        Ok(rate)
    }
}
