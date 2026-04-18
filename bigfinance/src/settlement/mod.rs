//! Settlement layer — USDC movement on Polygon.
//!
//! This is where USDC actually moves on-chain.
//! For now it's a stub — full implementation requires:
//!   - ethers-rs or alloy for EVM interaction
//!   - Polygon RPC connection
//!   - Custodial wallet management (Fireblocks or similar in production)
//!
//! chipin starts custodial (we hold the keys) and moves toward
//! non-custodial as the platform matures.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::config::Config;

pub struct SettlementLayer {
    config: Config,
}

impl SettlementLayer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Transfer USDC from user wallet to pool wallet.
    pub async fn deposit_to_pool(
        &self,
        from_user: &str,
        to_pool:   &str,
        amount:    Decimal,
    ) -> anyhow::Result<String> {
        // TODO: ERC-20 transfer via ethers-rs
        // USDC.transfer(to_pool_wallet, amount_in_6_decimals)
        tracing::info!(
            from = %from_user,
            to   = %to_pool,
            usdc = %amount,
            "USDC pool deposit TODO"
        );
        Ok(format!("0x{:064x}", 0)) // placeholder tx hash
    }

    /// Transfer USDC from pool wallet to user wallet (payout).
    pub async fn payout_from_pool(
        &self,
        from_pool: &str,
        to_user:   &str,
        amount:    Decimal,
    ) -> anyhow::Result<String> {
        // TODO: ERC-20 transfer
        tracing::info!(
            from = %from_pool,
            to   = %to_user,
            usdc = %amount,
            "USDC payout TODO"
        );
        Ok(format!("0x{:064x}", 0))
    }

    /// Format USDC amount for EVM (6 decimal places).
    /// e.g. 27.50 USDC → 27_500_000 (uint256)
    pub fn to_evm_units(amount: Decimal) -> u64 {
        (amount * dec!(1_000_000)).to_string()
            .split('.').next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    }

    /// Parse EVM units back to Decimal.
    pub fn from_evm_units(units: u64) -> Decimal {
        Decimal::from(units) / dec!(1_000_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_evm_units() {
        assert_eq!(SettlementLayer::to_evm_units(dec!(27.50)), 27_500_000);
        assert_eq!(SettlementLayer::to_evm_units(dec!(1.0)),   1_000_000);
        assert_eq!(SettlementLayer::from_evm_units(27_500_000), dec!(27.5));
    }
}
