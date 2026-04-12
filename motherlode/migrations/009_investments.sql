-- Investment stokvels
CREATE TYPE fund_type AS ENUM ('money_market', 'equity', 'bond', 'balanced');

CREATE TABLE investment_funds (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name            VARCHAR(255) NOT NULL,
    type            fund_type NOT NULL,
    avg_return_pct  NUMERIC(5,2) NOT NULL,
    min_amount      NUMERIC(12,2) NOT NULL,
    risk_level      VARCHAR(20) NOT NULL,  -- low | medium | high
    description     TEXT,
    is_active       BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE investment_positions (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    stokvel_id      UUID NOT NULL REFERENCES stokvels(id) ON DELETE CASCADE,
    fund_id         UUID NOT NULL REFERENCES investment_funds(id),
    amount_invested NUMERIC(12,2) NOT NULL DEFAULT 0,
    current_value   NUMERIC(12,2) NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE tax_reports (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID NOT NULL REFERENCES users(id),
    tax_year        INTEGER NOT NULL,
    interest_earned NUMERIC(12,2) NOT NULL DEFAULT 0,
    exemption_limit NUMERIC(12,2) NOT NULL DEFAULT 23800,
    report_url      TEXT,
    generated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, tax_year)
);
