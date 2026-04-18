//! Price oracle — fetches live USDC/local exchange rates.
//!
//! Primary:  Chainlink on-chain price feeds (tamper-proof)
//! Fallback: CoinGecko REST API
//! Dev:      Hardcoded rates (never in production)

pub mod chainlink;
pub mod coingecko;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::currency::currencies::SupportedCurrency;
use crate::currency::rates::{ExchangeRate, RateSource};
use crate::config::Config;
use chrono::Utc;

/// Fetch a live rate for a currency.
/// Tries Chainlink first, falls back to CoinGecko, then hardcoded dev rates.
pub async fn fetch_rate(
    currency: &SupportedCurrency,
    config: &Config,
) -> anyhow::Result<ExchangeRate> {
    // Try Chainlink (production)
    if !config.chainlink_rpc_url.is_empty() && config.is_production() {
        if let Ok(rate) = chainlink::fetch(currency, config).await {
            return Ok(rate);
        }
        tracing::warn!(
            currency = %currency.code(),
            "Chainlink fetch failed, falling back to CoinGecko"
        );
    }

    // Try CoinGecko (dev + Chainlink fallback)
    if !config.coingecko_api_key.is_empty() {
        if let Ok(rate) = coingecko::fetch(currency, config).await {
            return Ok(rate);
        }
        tracing::warn!(
            currency = %currency.code(),
            "CoinGecko fetch failed, using hardcoded dev rates"
        );
    }

    // Hardcoded dev rates — NEVER used in production
    if config.is_production() {
        anyhow::bail!(
            "All oracle sources failed for {} in production",
            currency.code()
        );
    }

    Ok(dev_rate(currency))
}

/// Hardcoded development rates — approximate, not for production.
fn dev_rate(currency: &SupportedCurrency) -> ExchangeRate {
    let usdc_to_local = match currency {
        SupportedCurrency::ZAR => dec!(18.50),
        SupportedCurrency::NGN => dec!(1650.0),
        SupportedCurrency::KES => dec!(130.0),
        SupportedCurrency::GHS => dec!(15.50),
        SupportedCurrency::INR => dec!(83.50),
        SupportedCurrency::PHP => dec!(56.50),
        SupportedCurrency::MXN => dec!(17.20),
        SupportedCurrency::BRL => dec!(5.05),
        SupportedCurrency::EGP => dec!(48.50),
        SupportedCurrency::USD => dec!(1.0),
        SupportedCurrency::EUR => dec!(0.92),
        SupportedCurrency::GBP => dec!(0.79),
        _                      => dec!(1.0),
    };

    let local_to_usdc = dec!(1) / usdc_to_local;

    ExchangeRate {
        usdc_to_local,
        local_to_usdc,
        currency:   currency.clone(),
        fetched_at: Utc::now(),
        source:     RateSource::Hardcoded,
    }
}
