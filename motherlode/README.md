# motherlode

StockFair backend API — Axum + SeaORM + PostgreSQL + Redis.

## Setup
```bash
cp .env.example .env
# fill in your values

sqlx database create
sqlx migrate run

cargo run
```

## Structure
See `src/` — routes → handlers → services → repositories.
Payment providers are fully swappable via the `PaymentProvider` trait.
