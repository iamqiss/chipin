//! Aave v3 integration on Polygon.
//!
//! Aave is the yield protocol of choice:
//!   - Battle-tested ($10B+ TVL)
//!   - USDC deposit → aUSDC (auto-compounding)
//!   - Typical APY: 3–8% depending on market conditions
//!   - Polygon = low gas fees (< $0.01 per tx)

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use crate::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AavePosition {
    /// USDC deposited into Aave
    pub principal_usdc:    Decimal,
    /// Current aUSDC balance (principal + accrued yield)
    pub current_value_usdc: Decimal,
    /// Current APY from Aave data provider
    pub current_apy:       Decimal,
    /// Total yield earned since deposit
    pub yield_earned_usdc: Decimal,
}

impl AavePosition {
    pub fn yield_earned(&self) -> Decimal {
        self.current_value_usdc - self.principal_usdc
    }
}

pub struct AaveClient {
    config: Config,
}

impl AaveClient {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Deposit USDC into Aave pool.
    /// Returns aUSDC balance after deposit.
    pub async fn deposit(&self, usdc_amount: Decimal) -> anyhow::Result<AavePosition> {
        // TODO: implement via ethers-rs / alloy
        // Call: AAVE_POOL.supply(USDC_ADDRESS, amount, on_behalf_of, referral_code)
        // Gas: ~$0.01 on Polygon

        tracing::info!(
            amount = %usdc_amount,
            pool = %self.config.aave_pool_address,
            "Aave deposit TODO"
        );

        // Stub response
        Ok(AavePosition {
            principal_usdc:    usdc_amount,
            current_value_usdc: usdc_amount,
            current_apy:       dec!(0.045), // 4.5% — typical USDC APY
            yield_earned_usdc: dec!(0),
        })
    }

    /// Withdraw USDC from Aave.
    pub async fn withdraw(&self, usdc_amount: Decimal) -> anyhow::Result<Decimal> {
        // TODO: Call: AAVE_POOL.withdraw(USDC_ADDRESS, amount, to)
        tracing::info!(amount = %usdc_amount, "Aave withdrawal TODO");
        Ok(usdc_amount)
    }

    /// Get current position (principal + yield).
    pub async fn get_position(&self, wallet: &str) -> anyhow::Result<AavePosition> {
        // TODO: Call AAVE_DATA_PROVIDER.getUserReserveData(USDC_ADDRESS, wallet)
        // Returns: aTokenBalance (= principal + yield), stableDebt, variableDebt, etc.
        tracing::info!(wallet = %wallet, "Aave position fetch TODO");
        Ok(AavePosition {
            principal_usdc:    dec!(0),
            current_value_usdc: dec!(0),
            current_apy:       dec!(0.045),
            yield_earned_usdc: dec!(0),
        })
    }

    /// Get current USDC supply APY from Aave.
    pub async fn get_current_apy(&self) -> anyhow::Result<Decimal> {
        // TODO: Call AAVE_DATA_PROVIDER.getReserveData(USDC_ADDRESS)
        // Returns: liquidityRate (ray units — divide by 1e27 for APY)
        Ok(dec!(0.045))
    }

    /// Check if current APY meets our minimum threshold.
    pub async fn meets_minimum_yield(&self) -> anyhow::Result<bool> {
        let apy = self.get_current_apy().await?;
        Ok(apy >= self.config.minimum_yield_apy_pct / dec!(100))
    }
}
