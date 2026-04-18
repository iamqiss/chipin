//! Fee calculator — applies the right fees for each operation type.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::config::Config;
use crate::currency::currencies::SupportedCurrency;
use super::FeeResult;

pub struct FeeCalculator {
    config: Config,
}

impl FeeCalculator {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Calculate fees for a member contribution.
    /// Platform fee applied. FX spread applied separately in converter.
    pub fn contribution(&self, gross_usdc: Decimal) -> FeeResult {
        let platform_fee = gross_usdc * Config::bps_to_decimal(self.config.platform_fee_bps);
        let net = gross_usdc - platform_fee;

        FeeResult {
            gross:          gross_usdc,
            net,
            platform_fee,
            fx_spread:      dec!(0), // applied separately in converter
            yield_perf_fee: dec!(0),
            withdrawal_fee: dec!(0),
            partner_cut:    dec!(0),
            total_fees:     platform_fee,
            currency:       SupportedCurrency::USDC,
        }
    }

    /// Calculate fees for a withdrawal.
    pub fn withdrawal(&self, gross_usdc: Decimal) -> FeeResult {
        let withdrawal_fee = gross_usdc * Config::bps_to_decimal(self.config.withdrawal_fee_bps);
        let net = gross_usdc - withdrawal_fee;

        FeeResult {
            gross:          gross_usdc,
            net,
            platform_fee:   dec!(0),
            fx_spread:      dec!(0),
            yield_perf_fee: dec!(0),
            withdrawal_fee,
            partner_cut:    dec!(0),
            total_fees:     withdrawal_fee,
            currency:       SupportedCurrency::USDC,
        }
    }

    /// Calculate chipin's performance fee on yield earned.
    /// Called monthly when yield is distributed.
    pub fn yield_performance(&self, gross_yield_usdc: Decimal) -> FeeResult {
        let perf_fee = gross_yield_usdc * Config::bps_to_decimal(self.config.yield_performance_fee_bps);
        let net = gross_yield_usdc - perf_fee;

        FeeResult {
            gross:          gross_yield_usdc,
            net,
            platform_fee:   dec!(0),
            fx_spread:      dec!(0),
            yield_perf_fee: perf_fee,
            withdrawal_fee: dec!(0),
            partner_cut:    dec!(0),
            total_fees:     perf_fee,
            currency:       SupportedCurrency::USDC,
        }
    }

    /// Calculate fees for a bulk Market order.
    /// Platform fee + partner cut both apply.
    pub fn market_order(&self, gross_usdc: Decimal) -> FeeResult {
        let platform_fee = gross_usdc * Config::bps_to_decimal(self.config.platform_fee_bps);
        let partner_cut  = gross_usdc * Config::bps_to_decimal(self.config.partner_cut_bps);
        let total_fees   = platform_fee + partner_cut;
        let net          = gross_usdc - total_fees;

        FeeResult {
            gross:          gross_usdc,
            net,
            platform_fee,
            fx_spread:      dec!(0),
            yield_perf_fee: dec!(0),
            withdrawal_fee: dec!(0),
            partner_cut,
            total_fees,
            currency:       SupportedCurrency::USDC,
        }
    }

    /// Calculate fees for a payout to a member.
    /// FX spread applied separately. Just the withdrawal fee here.
    pub fn payout(&self, gross_usdc: Decimal) -> FeeResult {
        self.withdrawal(gross_usdc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn test_config() -> Config {
        Config {
            host: "0.0.0.0".into(),
            port: 60051,
            app_env: "test".into(),
            redis_url: "redis://localhost".into(),
            chainlink_rpc_url: "".into(),
            coingecko_api_key: "".into(),
            rate_refresh_interval: 60,
            usdc_contract_address: "".into(),
            settlement_network: "polygon".into(),
            settlement_wallet_address: "".into(),
            settlement_wallet_key: "".into(),
            aave_pool_address: "".into(),
            aave_data_provider: "".into(),
            minimum_yield_apy_pct: dec!(3.5),
            platform_fee_bps: 50,
            yield_performance_fee_bps: 1000,
            withdrawal_fee_bps: 25,
            fx_spread_bps: 30,
            partner_cut_bps: 100,
        }
    }

    #[test]
    fn test_contribution_fee() {
        let calc = FeeCalculator::new(test_config());
        // R500 → USDC 27.027 (at 18.5 rate) → platform fee 0.5%
        let result = calc.contribution(dec!(27.027));
        assert_eq!(result.platform_fee.round_dp(4), dec!(0.1351));
        assert!(result.net < result.gross);
        assert_eq!(result.chipin_revenue(), result.platform_fee);
    }

    #[test]
    fn test_yield_performance_fee() {
        let calc = FeeCalculator::new(test_config());
        // Pool earns 100 USDC yield → chipin takes 10%
        let result = calc.yield_performance(dec!(100));
        assert_eq!(result.yield_perf_fee, dec!(10));
        assert_eq!(result.net, dec!(90));
    }

    #[test]
    fn test_market_order_fee() {
        let calc = FeeCalculator::new(test_config());
        // R1806 bulk order → USDC 97.62 → platform 0.5% + partner 1%
        let result = calc.market_order(dec!(97.62));
        assert_eq!(result.platform_fee.round_dp(4), dec!(0.4881));
        assert_eq!(result.partner_cut.round_dp(4), dec!(0.9762));
        assert_eq!(result.total_fees, result.platform_fee + result.partner_cut);
    }
}
