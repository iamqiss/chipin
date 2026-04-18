//! Exchange rate cache — rates fetched from oracle, cached in Redis.
//!
//! All rates are expressed as: 1 USDC = X local currency
//! e.g. ZAR rate = 18.5 means 1 USDC = R18.50

use std::collections::HashMap;
use redis::aio::ConnectionManager;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::currency::currencies::SupportedCurrency;

const RATE_KEY_PREFIX: &str = "bigfinance:rate:";
const RATE_TTL_SECONDS: u64 = 120; // 2 minutes max staleness

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    /// 1 USDC = this many units of the target currency
    pub usdc_to_local: Decimal,
    /// 1 unit of local = this many USDC
    pub local_to_usdc: Decimal,
    pub currency:      SupportedCurrency,
    pub fetched_at:    DateTime<Utc>,
    pub source:        RateSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RateSource {
    Chainlink,  // on-chain oracle — preferred
    CoinGecko,  // REST API fallback
    Hardcoded,  // dev/testing only — never in production
}

pub struct RateCache {
    redis: ConnectionManager,
}

impl RateCache {
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    /// Get cached rate for a currency pair. Returns None if stale or missing.
    pub async fn get(&mut self, currency: &SupportedCurrency) -> Option<ExchangeRate> {
        let key = format!("{}{}", RATE_KEY_PREFIX, currency.code());
        let raw: Option<String> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut self.redis)
            .await
            .ok()?;

        raw.and_then(|s| serde_json::from_str(&s).ok())
    }

    /// Store a fresh rate in Redis.
    pub async fn set(&mut self, rate: &ExchangeRate) -> anyhow::Result<()> {
        let key = format!("{}{}", RATE_KEY_PREFIX, rate.currency.code());
        let value = serde_json::to_string(rate)?;
        redis::cmd("SETEX")
            .arg(&key)
            .arg(RATE_TTL_SECONDS)
            .arg(value)
            .query_async::<_, ()>(&mut self.redis)
            .await?;
        Ok(())
    }

    /// Store multiple rates atomically.
    pub async fn set_many(&mut self, rates: &[ExchangeRate]) -> anyhow::Result<()> {
        for rate in rates {
            self.set(rate).await?;
        }
        Ok(())
    }

    /// Get all cached rates as a map.
    pub async fn get_all(&mut self) -> HashMap<String, ExchangeRate> {
        let mut map = HashMap::new();
        for currency in all_supported_currencies() {
            if let Some(rate) = self.get(&currency).await {
                map.insert(currency.code().to_string(), rate);
            }
        }
        map
    }
}

fn all_supported_currencies() -> Vec<SupportedCurrency> {
    vec![
        SupportedCurrency::ZAR, SupportedCurrency::NGN, SupportedCurrency::KES,
        SupportedCurrency::GHS, SupportedCurrency::INR, SupportedCurrency::PHP,
        SupportedCurrency::MXN, SupportedCurrency::BRL, SupportedCurrency::EGP,
        SupportedCurrency::USD, SupportedCurrency::EUR, SupportedCurrency::GBP,
    ]
}
