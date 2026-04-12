-- User wallets & transactions
CREATE TYPE tx_type AS ENUM ('deposit', 'withdrawal', 'contribution', 'payout', 'transfer', 'refund', 'fee');
CREATE TYPE tx_status AS ENUM ('pending', 'completed', 'failed', 'reversed');

CREATE TABLE user_wallets (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id     UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    balance     NUMERIC(12,2) NOT NULL DEFAULT 0,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE transactions (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID NOT NULL REFERENCES users(id),
    type            tx_type NOT NULL,
    amount          NUMERIC(12,2) NOT NULL,
    fee             NUMERIC(12,2) NOT NULL DEFAULT 0,
    status          tx_status NOT NULL DEFAULT 'pending',
    reference       TEXT UNIQUE,
    provider_ref    TEXT,
    stokvel_id      UUID REFERENCES stokvels(id),
    metadata        JSONB,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE linked_bank_accounts (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    account_name    VARCHAR(255) NOT NULL,
    account_number  VARCHAR(50) NOT NULL,
    bank_name       VARCHAR(100) NOT NULL,
    branch_code     VARCHAR(20),
    is_primary      BOOLEAN NOT NULL DEFAULT false,
    is_verified     BOOLEAN NOT NULL DEFAULT false,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
