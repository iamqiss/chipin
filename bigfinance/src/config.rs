//! Configuration — loaded from environment variables.

use rust_decimal::Decimal;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Config {
    // Server
    pub host: String,
    pub port: u16,
    pub app_env: String,

    // Redis
    pub redis_url: String,

    // Oracle
    pub chainlink_rpc_url:    String,
    pub coingecko_api_key:    String,
    pub rate_refresh_interval: u64,

    // USDC
    pub usdc_contract_address:    String,
    pub settlement_network:       String,
    pub settlement_wallet_address: String,
    pub settlement_wallet_key:    String,

    // DeFi
    pub aave_pool_address:      String,
    pub aave_data_provider:     String,
    pub minimum_yield_apy_pct:  Decimal,

    // Fees (basis points)
    pub platform_fee_bps:          u32,
    pub yield_performance_fee_bps: u32,
    pub withdrawal_fee_bps:        u32,
    pub fx_spread_bps:             u32,
    pub partner_cut_bps:           u32,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            host:     std::env::var("BIGFINANCE_HOST").unwrap_or("0.0.0.0".into()),
            port:     std::env::var("BIGFINANCE_PORT").unwrap_or("60051".into()).parse()?,
            app_env:  std::env::var("APP_ENV").unwrap_or("development".into()),
            redis_url: std::env::var("REDIS_URL")?,
            chainlink_rpc_url:    std::env::var("CHAINLINK_RPC_URL").unwrap_or_default(),
            coingecko_api_key:    std::env::var("COINGECKO_API_KEY").unwrap_or_default(),
            rate_refresh_interval: std::env::var("RATE_REFRESH_INTERVAL").unwrap_or("60".into()).parse()?,
            usdc_contract_address:    std::env::var("USDC_CONTRACT_ADDRESS").unwrap_or_default(),
            settlement_network:       std::env::var("SETTLEMENT_NETWORK").unwrap_or("polygon".into()),
            settlement_wallet_address: std::env::var("SETTLEMENT_WALLET_ADDRESS").unwrap_or_default(),
            settlement_wallet_key:    std::env::var("SETTLEMENT_WALLET_KEY").unwrap_or_default(),
            aave_pool_address:    std::env::var("AAVE_POOL_ADDRESS").unwrap_or_default(),
            aave_data_provider:   std::env::var("AAVE_DATA_PROVIDER").unwrap_or_default(),
            minimum_yield_apy_pct: std::env::var("MINIMUM_YIELD_APY_PCT")
                .unwrap_or("3.5".into()).parse()?,
            platform_fee_bps:          std::env::var("PLATFORM_FEE_BPS").unwrap_or("50".into()).parse()?,
            yield_performance_fee_bps: std::env::var("YIELD_PERFORMANCE_FEE_BPS").unwrap_or("1000".into()).parse()?,
            withdrawal_fee_bps:        std::env::var("WITHDRAWAL_FEE_BPS").unwrap_or("25".into()).parse()?,
            fx_spread_bps:             std::env::var("FX_SPREAD_BPS").unwrap_or("30".into()).parse()?,
            partner_cut_bps:           std::env::var("PARTNER_CUT_BPS").unwrap_or("100".into()).parse()?,
        })
    }

    pub fn is_production(&self) -> bool {
        self.app_env == "production"
    }

    /// Convert basis points to a decimal multiplier.
    /// e.g. 50 bps → 0.005
    pub fn bps_to_decimal(bps: u32) -> Decimal {
        Decimal::new(bps as i64, 4)
    }
}
