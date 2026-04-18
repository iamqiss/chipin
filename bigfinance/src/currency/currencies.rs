//! All currencies chipin supports.
//!
//! USDC is the internal settlement layer — users never see it.
//! Add new currencies here to expand to new markets.

use serde::{Deserialize, Serialize};

/// Every currency chipin knows about.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "uppercase")]
pub enum SupportedCurrency {
    // ── Internal settlement ───────────────────────────────────────────────
    USDC,  // always the base — users never see this

    // ── Africa ────────────────────────────────────────────────────────────
    ZAR,   // South Africa — launch market
    NGN,   // Nigeria
    KES,   // Kenya
    GHS,   // Ghana
    TZS,   // Tanzania
    UGX,   // Uganda
    ZMW,   // Zambia
    MWK,   // Malawi
    BWP,   // Botswana
    NAD,   // Namibia

    // ── Asia ──────────────────────────────────────────────────────────────
    INR,   // India (chit funds)
    PHP,   // Philippines
    IDR,   // Indonesia
    VND,   // Vietnam
    BDT,   // Bangladesh

    // ── Latin America ─────────────────────────────────────────────────────
    MXN,   // Mexico (tandas)
    BRL,   // Brazil
    COP,   // Colombia
    PEN,   // Peru
    CLP,   // Chile

    // ── Middle East / North Africa ────────────────────────────────────────
    EGP,   // Egypt (gameya)
    MAD,   // Morocco
    TND,   // Tunisia

    // ── Global fallback ───────────────────────────────────────────────────
    USD,   // United States
    EUR,   // Europe
    GBP,   // United Kingdom
}

impl SupportedCurrency {
    pub fn code(&self) -> &'static str {
        match self {
            Self::USDC => "USDC",
            Self::ZAR  => "ZAR",
            Self::NGN  => "NGN",
            Self::KES  => "KES",
            Self::GHS  => "GHS",
            Self::TZS  => "TZS",
            Self::UGX  => "UGX",
            Self::ZMW  => "ZMW",
            Self::MWK  => "MWK",
            Self::BWP  => "BWP",
            Self::NAD  => "NAD",
            Self::INR  => "INR",
            Self::PHP  => "PHP",
            Self::IDR  => "IDR",
            Self::VND  => "VND",
            Self::BDT  => "BDT",
            Self::MXN  => "MXN",
            Self::BRL  => "BRL",
            Self::COP  => "COP",
            Self::PEN  => "PEN",
            Self::CLP  => "CLP",
            Self::EGP  => "EGP",
            Self::MAD  => "MAD",
            Self::TND  => "TND",
            Self::USD  => "USD",
            Self::EUR  => "EUR",
            Self::GBP  => "GBP",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Self::USDC => "$",
            Self::ZAR  => "R",
            Self::NGN  => "₦",
            Self::KES  => "KSh",
            Self::GHS  => "GH₵",
            Self::TZS  => "TSh",
            Self::UGX  => "USh",
            Self::ZMW  => "ZK",
            Self::MWK  => "MK",
            Self::BWP  => "P",
            Self::NAD  => "N$",
            Self::INR  => "₹",
            Self::PHP  => "₱",
            Self::IDR  => "Rp",
            Self::VND  => "₫",
            Self::BDT  => "৳",
            Self::MXN  => "MX$",
            Self::BRL  => "R$",
            Self::COP  => "COL$",
            Self::PEN  => "S/",
            Self::CLP  => "CL$",
            Self::EGP  => "£",
            Self::MAD  => "MAD",
            Self::TND  => "DT",
            Self::USD  => "$",
            Self::EUR  => "€",
            Self::GBP  => "£",
        }
    }

    /// Decimal places used for display.
    pub fn decimals(&self) -> u32 {
        match self {
            Self::VND | Self::IDR | Self::UGX | Self::TZS => 0,
            _ => 2,
        }
    }

    /// Format an amount in this currency for display.
    /// Users always see local currency — USDC is hidden.
    pub fn format(&self, amount: rust_decimal::Decimal) -> String {
        let decimals = self.decimals();
        let rounded  = amount.round_dp(decimals);
        format!("{}{}", self.symbol(), rounded)
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_uppercase().as_str() {
            "USDC" => Some(Self::USDC),
            "ZAR"  => Some(Self::ZAR),
            "NGN"  => Some(Self::NGN),
            "KES"  => Some(Self::KES),
            "GHS"  => Some(Self::GHS),
            "INR"  => Some(Self::INR),
            "MXN"  => Some(Self::MXN),
            "EGP"  => Some(Self::EGP),
            "USD"  => Some(Self::USD),
            "EUR"  => Some(Self::EUR),
            "GBP"  => Some(Self::GBP),
            _ => None,
        }
    }
}

/// A money value with its currency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub amount:   rust_decimal::Decimal,
    pub currency: SupportedCurrency,
}

impl Currency {
    pub fn new(amount: rust_decimal::Decimal, currency: SupportedCurrency) -> Self {
        Self { amount, currency }
    }

    pub fn zar(amount: rust_decimal::Decimal) -> Self {
        Self::new(amount, SupportedCurrency::ZAR)
    }

    pub fn usdc(amount: rust_decimal::Decimal) -> Self {
        Self::new(amount, SupportedCurrency::USDC)
    }

    pub fn format(&self) -> String {
        self.currency.format(self.amount)
    }
}
