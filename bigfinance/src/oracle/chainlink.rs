//! Chainlink on-chain price feed integration.
//! Fetches USD price of each local currency from Chainlink's decentralized oracles.
//! Production only — requires RPC connection to Polygon.

use rust_decimal::Decimal;
use chrono::Utc;
use crate::currency::currencies::SupportedCurrency;
use crate::currency::rates::{ExchangeRate, RateSource};
use crate::config::Config;

/// Chainlink feed addresses on Polygon mainnet.
/// All feeds return local/USD price (how many USD per 1 unit of local).
fn feed_address(currency: &SupportedCurrency) -> Option<&'static str> {
    match currency {
        SupportedCurrency::ZAR => Some("0x..."), // ZAR/USD feed — TODO: real address
        SupportedCurrency::NGN => Some("0x..."), // NGN/USD feed
        SupportedCurrency::KES => Some("0x..."), // KES/USD feed
        SupportedCurrency::INR => Some("0x..."), // INR/USD feed
        SupportedCurrency::MXN => Some("0x..."), // MXN/USD feed
        SupportedCurrency::EGP => Some("0x..."), // EGP/USD feed
        SupportedCurrency::EUR => Some("0x..."), // EUR/USD feed
        SupportedCurrency::GBP => Some("0x..."), // GBP/USD feed
        _                      => None,
    }
}

pub async fn fetch(
    currency: &SupportedCurrency,
    config: &Config,
) -> anyhow::Result<ExchangeRate> {
    let _feed = feed_address(currency)
        .ok_or_else(|| anyhow::anyhow!("No Chainlink feed for {}", currency.code()))?;

    // TODO: implement ethers-rs / alloy call to Chainlink AggregatorV3Interface
    // Interface: latestRoundData() returns (roundId, answer, startedAt, updatedAt, answeredInRound)
    // answer is the price with 8 decimal places

    // Placeholder until ethers integration is complete
    anyhow::bail!("Chainlink integration TODO — use CoinGecko fallback for now")
}
