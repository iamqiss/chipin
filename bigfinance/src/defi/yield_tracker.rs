//! Yield tracking per pool and per member.
//!
//! Tracks how much yield each pool has earned and
//! calculates each member's pro-rata share.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolYieldSnapshot {
    pub pool_id:           String,
    pub principal_usdc:    Decimal,
    pub current_usdc:      Decimal,
    pub yield_earned_usdc: Decimal,
    pub apy:               Decimal,
    pub snapshot_at:       chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberYieldShare {
    pub user_id:        String,
    pub pool_id:        String,
    /// Member's share of the pool (0.0 - 1.0)
    pub pool_share:     Decimal,
    /// Member's gross yield in USDC
    pub gross_yield:    Decimal,
    /// chipin's performance fee (10%)
    pub perf_fee:       Decimal,
    /// Member's net yield after fee
    pub net_yield:      Decimal,
}

pub struct YieldTracker;

impl YieldTracker {
    /// Calculate each member's yield share from a pool snapshot.
    ///
    /// Distribution is pro-rata by contribution amount.
    /// All members who contributed get a share proportional to
    /// how much they put in — equal power regardless of wealth.
    pub fn calculate_member_shares(
        snapshot: &PoolYieldSnapshot,
        member_contributions: &HashMap<String, Decimal>, // user_id → USDC contributed
        performance_fee_bps: u32,
    ) -> Vec<MemberYieldShare> {
        let total_contributed: Decimal = member_contributions.values().sum();
        if total_contributed == dec!(0) {
            return vec![];
        }

        let perf_fee_rate = crate::config::Config::bps_to_decimal(performance_fee_bps);
        let gross_yield   = snapshot.yield_earned_usdc;

        member_contributions.iter().map(|(user_id, contribution)| {
            let pool_share  = contribution / total_contributed;
            let member_gross = gross_yield * pool_share;
            let perf_fee    = member_gross * perf_fee_rate;
            let net_yield   = member_gross - perf_fee;

            MemberYieldShare {
                user_id:    user_id.clone(),
                pool_id:    snapshot.pool_id.clone(),
                pool_share,
                gross_yield: member_gross,
                perf_fee,
                net_yield,
            }
        }).collect()
    }

    /// Annualised yield rate for display (e.g. "4.5% p.a.")
    pub fn annualised_apy(
        principal: Decimal,
        yield_earned: Decimal,
        days_elapsed: u32,
    ) -> Decimal {
        if principal == dec!(0) || days_elapsed == 0 {
            return dec!(0);
        }
        let daily_rate = yield_earned / principal / Decimal::from(days_elapsed);
        daily_rate * dec!(365) * dec!(100) // as percentage
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use chrono::Utc;

    #[test]
    fn test_equal_contribution_equal_yield() {
        let snapshot = PoolYieldSnapshot {
            pool_id:           "pool1".into(),
            principal_usdc:    dec!(1000),
            current_usdc:      dec!(1045),
            yield_earned_usdc: dec!(45),
            apy:               dec!(0.045),
            snapshot_at:       Utc::now(),
        };

        let mut contributions = HashMap::new();
        contributions.insert("alice".into(), dec!(500));
        contributions.insert("bob".into(),   dec!(500));

        let shares = YieldTracker::calculate_member_shares(&snapshot, &contributions, 1000);

        // Each should get 50% of the yield minus 10% perf fee
        for share in &shares {
            assert_eq!(share.pool_share, dec!(0.5));
            assert_eq!(share.gross_yield, dec!(22.5));  // 50% of 45
            assert_eq!(share.perf_fee,    dec!(2.25));  // 10% of 22.5
            assert_eq!(share.net_yield,   dec!(20.25)); // gross - perf_fee
        }
    }

    #[test]
    fn test_unequal_contribution_proportional_yield() {
        let snapshot = PoolYieldSnapshot {
            pool_id:           "pool2".into(),
            principal_usdc:    dec!(900),
            current_usdc:      dec!(940.5),
            yield_earned_usdc: dec!(40.5),
            apy:               dec!(0.045),
            snapshot_at:       Utc::now(),
        };

        let mut contributions = HashMap::new();
        contributions.insert("big".into(),   dec!(600)); // 2/3
        contributions.insert("small".into(), dec!(300)); // 1/3

        let shares = YieldTracker::calculate_member_shares(&snapshot, &contributions, 1000);
        let big   = shares.iter().find(|s| s.user_id == "big").unwrap();
        let small = shares.iter().find(|s| s.user_id == "small").unwrap();

        // Big contributor gets 2x the yield of small
        assert!((big.net_yield - small.net_yield * dec!(2)).abs() < dec!(0.001));
    }
}
