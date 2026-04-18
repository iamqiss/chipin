//! bigfinance — chipin's DeFi + multi-currency engine.
//!
//! Every financial operation in chipin flows through here before
//! motherlode commits anything to the database.
//!
//! # Flow
//! ```text
//! User action (contribute R500)
//!     ↓
//! motherlode receives request
//!     ↓
//! bigfinance::pipeline::process(contribution)
//!     ↓ converts ZAR → USDC at live rate
//!     ↓ deducts platform fee (0.5%)
//!     ↓ routes net amount to yield protocol (Aave)
//!     ↓ records revenue split
//!     ↓ returns settlement confirmation
//!     ↓
//! motherlode writes confirmed transaction to DB
//! ```

pub mod config;
pub mod currency;
pub mod defi;
pub mod fees;
pub mod pipeline;
pub mod revenue;
pub mod settlement;
pub mod oracle;
pub mod proto;
pub mod server;
