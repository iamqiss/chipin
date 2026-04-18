//! Fee engine — every chipin revenue stream lives here.
//!
//! Fee hierarchy (applied in order):
//!   1. Platform fee        — on every contribution (0.5%)
//!   2. FX spread           — on every currency conversion (0.3%)
//!   3. Yield performance   — on DeFi yield earned (10% of yield)
//!   4. Withdrawal fee      — on every withdrawal (0.25%)
//!   5. Partner cut         — on bulk Market orders (1% to retailer)
//!
//! All fees are denominated in USDC internally.
//! Users see amounts in local currency — fee deduction is transparent
//! but the USDC mechanics are hidden.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::currency::currencies::{Currency, SupportedCurrency};

pub mod calculator;
pub use calculator::FeeCalculator;

/// The result of fee calculation for a single operation.
#[derive(Debug, Clone, Serialize)]
pub struct FeeResult {
    /// Gross amount before any fees
    pub gross:              Decimal,
    /// Net amount after all applicable fees
    pub net:                Decimal,
    /// Platform fee (chipin revenue)
    pub platform_fee:       Decimal,
    /// FX spread (chipin revenue on conversion)
    pub fx_spread:          Decimal,
    /// Yield performance fee (chipin's cut of yield)
    pub yield_perf_fee:     Decimal,
    /// Withdrawal fee (chipin revenue)
    pub withdrawal_fee:     Decimal,
    /// Partner cut (paid out to retail partner)
    pub partner_cut:        Decimal,
    /// Total fees deducted
    pub total_fees:         Decimal,
    /// Currency all amounts are in (always USDC internally)
    pub currency:           SupportedCurrency,
}

impl FeeResult {
    /// chipin's total revenue from this operation (excludes partner cut).
    pub fn chipin_revenue(&self) -> Decimal {
        self.platform_fee + self.fx_spread + self.yield_perf_fee + self.withdrawal_fee
    }

    /// Total going out to partners.
    pub fn partner_revenue(&self) -> Decimal {
        self.partner_cut
    }

    pub fn summary(&self) -> String {
        format!(
            "gross={} net={} fees={} chipin_rev={} partner={}",
            self.gross, self.net, self.total_fees,
            self.chipin_revenue(), self.partner_cut
        )
    }
}
