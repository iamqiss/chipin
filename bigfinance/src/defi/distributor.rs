//! Yield distributor — sends yield to members monthly.
//! Called by the background job in motherlode.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::defi::yield_tracker::{MemberYieldShare, YieldTracker, PoolYieldSnapshot};
use std::collections::HashMap;

pub struct YieldDistributor;

impl YieldDistributor {
    /// Distribute yield for a pool to all members.
    /// Returns the distribution plan — motherlode executes the actual payments.
    pub fn plan(
        snapshot: &PoolYieldSnapshot,
        member_contributions: &HashMap<String, Decimal>,
        performance_fee_bps: u32,
    ) -> DistributionPlan {
        let shares = YieldTracker::calculate_member_shares(
            snapshot,
            member_contributions,
            performance_fee_bps,
        );

        let chipin_revenue: Decimal = shares.iter().map(|s| s.perf_fee).sum();
        let total_distributed: Decimal = shares.iter().map(|s| s.net_yield).sum();

        DistributionPlan {
            pool_id:          snapshot.pool_id.clone(),
            total_yield:      snapshot.yield_earned_usdc,
            chipin_revenue,
            total_distributed,
            shares,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DistributionPlan {
    pub pool_id:          String,
    pub total_yield:      Decimal,
    pub chipin_revenue:   Decimal,
    pub total_distributed: Decimal,
    pub shares:           Vec<MemberYieldShare>,
}

impl DistributionPlan {
    pub fn verify(&self) -> bool {
        // Conservation check: total_yield = chipin_revenue + total_distributed
        let sum = self.chipin_revenue + self.total_distributed;
        (sum - self.total_yield).abs() < dec!(0.000001)
    }
}
