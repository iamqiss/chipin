//! DeFi yield engine.
//!
//! chipin deposits pool USDC into Aave v3 on Polygon.
//! Users see nothing — their money just grows while it sits.
//!
//! Flow:
//!   Pool receives contribution (USDC)
//!       ↓ bigfinance routes to Aave
//!   Aave issues aUSDC (interest-bearing token)
//!       ↓ yield accrues per second
//!   Monthly: bigfinance harvests yield
//!       ↓ applies performance fee (10%)
//!       ↓ distributes remaining to members pro-rata
//!   Member receives yield in local currency at payout

pub mod aave;
pub mod yield_tracker;
pub mod distributor;

pub use yield_tracker::YieldTracker;
pub use distributor::YieldDistributor;
