//! Revenue accounting — every cent chipin earns is tracked here.
//!
//! Revenue streams:
//!   1. Platform fee       (0.5% per contribution)
//!   2. FX spread          (0.3% per conversion)
//!   3. Yield performance  (10% of DeFi yield)
//!   4. Withdrawal fee     (0.25% per withdrawal)
//!   5. Partner cut        (1% of bulk orders — flows to retailer)
//!
//! This module provides the ledger.
//! motherlode writes the actual DB records.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RevenueStream {
    PlatformFee,
    FxSpread,
    YieldPerformance,
    WithdrawalFee,
    PartnerCut { retailer_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueEvent {
    pub id:          String,
    pub stream:      RevenueStream,
    pub amount_usdc: Decimal,
    pub reference:   String,  // transaction reference
    pub pool_id:     Option<String>,
    pub user_id:     Option<String>,
    pub recorded_at: DateTime<Utc>,
}

/// Revenue recorder — emits events for motherlode to persist.
pub struct RevenueRecorder {
    events: Vec<RevenueEvent>,
}

impl RevenueRecorder {
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    pub fn record(
        &mut self,
        stream: RevenueStream,
        amount_usdc: Decimal,
        reference: &str,
        pool_id: Option<&str>,
        user_id: Option<&str>,
    ) {
        if amount_usdc <= dec!(0) { return; }
        self.events.push(RevenueEvent {
            id:          uuid::Uuid::new_v4().to_string(),
            stream,
            amount_usdc,
            reference:   reference.to_string(),
            pool_id:     pool_id.map(|s| s.to_string()),
            user_id:     user_id.map(|s| s.to_string()),
            recorded_at: Utc::now(),
        });
    }

    pub fn drain(&mut self) -> Vec<RevenueEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn total(&self) -> Decimal {
        self.events.iter().map(|e| e.amount_usdc).sum()
    }
}
