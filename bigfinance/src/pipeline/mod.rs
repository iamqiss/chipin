//! Transaction pipeline — the main entry point.
//!
//! Every financial operation in chipin passes through here.
//! motherlode calls bigfinance BEFORE writing to the database.
//!
//! The pipeline:
//!   1. Validate the operation
//!   2. Convert currency (local → USDC)
//!   3. Apply fees (platform, FX spread)
//!   4. Route to DeFi (Aave deposit)
//!   5. Record revenue split
//!   6. Return settlement confirmation to motherlode

pub mod contribution;
pub mod withdrawal;
pub mod payout;
pub mod market_order;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::currency::currencies::{Currency, SupportedCurrency};

/// A settled transaction — returned to motherlode after bigfinance processes it.
/// motherlode writes this to the database as confirmation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementResult {
    /// chipin's internal reference
    pub reference:        String,
    /// User's local currency input
    pub local_input:      Currency,
    /// USDC amount after conversion + fees
    pub usdc_settled:     Decimal,
    /// chipin revenue from this transaction (USDC)
    pub chipin_revenue:   Decimal,
    /// Partner revenue if applicable (USDC)
    pub partner_revenue:  Decimal,
    /// Whether funds are in Aave generating yield
    pub is_in_yield:      bool,
    /// Current APY if in yield
    pub current_apy:      Option<Decimal>,
    /// Settlement timestamp
    pub settled_at:       String,
}
