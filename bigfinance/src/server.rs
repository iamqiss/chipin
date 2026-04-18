//! bigfinance server — can run as:
//!   1. Embedded library (called directly from motherlode in the same process)
//!   2. Standalone gRPC service (separate process for scale)
//!
//! Start with option 1 for simplicity, move to 2 when scaling.

use crate::config::Config;
use crate::currency::rates::RateCache;
use crate::oracle;

pub async fn run(config: Config) -> anyhow::Result<()> {
    // Connect Redis for rate cache
    let redis_client = redis::Client::open(config.redis_url.clone())?;
    let redis        = redis::aio::ConnectionManager::new(redis_client).await?;
    let mut cache    = RateCache::new(redis.clone());

    // Pre-warm rate cache on startup
    tracing::info!("Pre-warming rate cache...");
    let currencies = vec![
        crate::currency::currencies::SupportedCurrency::ZAR,
        crate::currency::currencies::SupportedCurrency::NGN,
        crate::currency::currencies::SupportedCurrency::KES,
        crate::currency::currencies::SupportedCurrency::INR,
        crate::currency::currencies::SupportedCurrency::MXN,
        crate::currency::currencies::SupportedCurrency::EGP,
    ];

    for currency in &currencies {
        match oracle::fetch_rate(currency, &config).await {
            Ok(rate) => {
                tracing::info!(
                    currency = %currency.code(),
                    rate     = %rate.usdc_to_local,
                    "Rate cached"
                );
                cache.set(&rate).await.ok();
            }
            Err(e) => {
                tracing::warn!(currency = %currency.code(), error = %e, "Rate fetch failed");
            }
        }
    }

    // Spawn background rate refresher
    let config_clone = config.clone();
    let redis_clone  = redis.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(
            std::time::Duration::from_secs(config_clone.rate_refresh_interval)
        );
        let mut cache = RateCache::new(redis_clone);
        loop {
            interval.tick().await;
            for currency in &currencies {
                if let Ok(rate) = oracle::fetch_rate(currency, &config_clone).await {
                    cache.set(&rate).await.ok();
                }
            }
        }
    });

    tracing::info!("bigfinance ready — rates refreshing every {}s", config.rate_refresh_interval);

    // TODO: start gRPC server when bigfinance.proto is defined
    // For now: block forever (background tasks run)
    tokio::signal::ctrl_c().await?;
    tracing::info!("bigfinance shutting down");

    Ok(())
}
