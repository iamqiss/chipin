-- Contributions & payouts
CREATE TYPE contribution_status AS ENUM ('pending', 'paid', 'overdue', 'waived');
CREATE TYPE payout_status AS ENUM ('scheduled', 'processing', 'completed', 'failed');

CREATE TABLE contributions (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    stokvel_id      UUID NOT NULL REFERENCES stokvels(id) ON DELETE CASCADE,
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount          NUMERIC(12,2) NOT NULL,
    due_date        DATE NOT NULL,
    paid_at         TIMESTAMPTZ,
    status          contribution_status NOT NULL DEFAULT 'pending',
    fine_amount     NUMERIC(12,2) NOT NULL DEFAULT 0,
    payment_ref     TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE payouts (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    stokvel_id      UUID NOT NULL REFERENCES stokvels(id) ON DELETE CASCADE,
    recipient_id    UUID NOT NULL REFERENCES users(id),
    amount          NUMERIC(12,2) NOT NULL,
    scheduled_for   DATE NOT NULL,
    paid_at         TIMESTAMPTZ,
    status          payout_status NOT NULL DEFAULT 'scheduled',
    payment_ref     TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE auto_pay_settings (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    stokvel_id      UUID NOT NULL REFERENCES stokvels(id) ON DELETE CASCADE,
    is_enabled      BOOLEAN NOT NULL DEFAULT false,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, stokvel_id)
);
