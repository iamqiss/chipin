//! CoinGecko REST API rate fetcher.
//! Fallback when Chainlink is unavailable.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::str::FromStr;
use chrono::Utc;
use serde::Deserialize;
use crate::currency::currencies::SupportedCurrency;
use crate::currency::rates::{ExchangeRate, RateSource};
use crate::config::Config;

/// CoinGecko currency IDs for USD price.
fn coingecko_vs_currency(currency: &SupportedCurrency) -> Option<&'static str> {
    match currency {
        SupportedCurrency::ZAR => Some("zar"),
        SupportedCurrency::NGN => Some("ngn"),
        SupportedCurrency::KES => Some("kes"),
        SupportedCurrency::GHS => Some("ghs"),
        SupportedCurrency::INR => Some("inr"),
        SupportedCurrency::PHP => Some("php"),
        SupportedCurrency::MXN => Some("mxn"),
        SupportedCurrency::BRL => Some("brl"),
        SupportedCurrency::EGP => Some("egp"),
        SupportedCurrency::USD => Some("usd"),
        SupportedCurrency::EUR => Some("eur"),
        SupportedCurrency::GBP => Some("gbp"),
        _                      => None,
    }
}

#[derive(Deserialize)]
struct CoinGeckoResponse {
    #[serde(rename = "usd-coin")]
    usdc: std::collections::HashMap<String, f64>,
}

pub async fn fetch(
    currency: &SupportedCurrency,
    config: &Config,
) -> anyhow::Result<ExchangeRate> {
    let vs = coingecko_vs_currency(currency)
        .ok_or_else(|| anyhow::anyhow!("No CoinGecko mapping for {}", currency.code()))?;

    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids=usd-coin&vs_currencies={}",
        vs
    );

    let client = reqwest::Client::new();
    let mut req = client.get(&url);

    if !config.coingecko_api_key.is_empty() {
        req = req.header("x-cg-demo-api-key", &config.coingecko_api_key);
    }

    let resp: CoinGeckoResponse = req.send().await?.json().await?;

    let rate_f64 = resp.usdc.get(vs)
        .ok_or_else(|| anyhow::anyhow!("CoinGecko missing {} rate", vs))?;

    let usdc_to_local = Decimal::from_str(&rate_f64.to_string())?;
    let local_to_usdc = dec!(1) / usdc_to_local;

    tracing::info!(
        currency = %currency.code(),
        rate = %usdc_to_local,
        "CoinGecko rate fetched"
    );

    Ok(ExchangeRate {
        usdc_to_local,
        local_to_usdc,
        currency:   currency.clone(),
        fetched_at: Utc::now(),
        source:     RateSource::CoinGecko,
    })
}
