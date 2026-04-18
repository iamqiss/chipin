//! Multi-currency support.
//!
//! chipin's internal settlement currency is USDC.
//! Users always see and interact in their local currency.
//! All conversions happen here, invisibly.

pub mod currencies;
pub mod converter;
pub mod rates;

pub use currencies::{Currency, SupportedCurrency};
pub use converter::CurrencyConverter;
pub use rates::RateCache;
