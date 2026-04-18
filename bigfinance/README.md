# bigfinance

chipin's DeFi + multi-currency + fee engine.

Every financial operation in chipin passes through bigfinance **before** motherlode touches the database. This is the money brain.

## What lives here

| Module | Responsibility |
|---|---|
| `currency/` | 28 currencies, USDC as internal settlement layer |
| `oracle/` | Live exchange rates via Chainlink (prod) + CoinGecko (fallback) |
| `fees/` | Platform fee, FX spread, yield performance, withdrawal fee, partner cut |
| `defi/` | Aave v3 deposit/withdraw, yield tracking, pro-rata distribution |
| `pipeline/` | Contribution, withdrawal, payout, market order flows |
| `revenue/` | Revenue ledger — every chipin earning tracked |
| `settlement/` | USDC on-chain movement (Polygon) |

## Fee structure

| Stream | Rate | Applied on |
|---|---|---|
| Platform fee | 0.5% | Every contribution |
| FX spread | 0.3% | Every currency conversion |
| Yield performance | 10% | Monthly yield earned |
| Withdrawal fee | 0.25% | Every withdrawal |
| Partner cut | 1% | Bulk Market orders (paid to retailer) |

## Never use f64 for money

All amounts use `rust_decimal::Decimal`. No floating point. Ever.

## DeFi yield flow

```
Member chips in R500 (ZAR)
    ↓ convert ZAR → USDC at live rate (0.3% spread)
    ↓ deduct platform fee (0.5%)
    ↓ deposit net USDC into Aave v3 on Polygon
Pool earns ~4.5% APY while it sits
    ↓ monthly harvest
    ↓ chipin takes 10% performance fee
    ↓ remainder distributed pro-rata to members
    ↓ convert USDC → local currency at payout (0.3% spread + 0.25% withdrawal)
Member receives local currency — never sees USDC
```
