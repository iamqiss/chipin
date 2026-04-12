-- Stokvels & membership
CREATE TYPE stokvel_type AS ENUM ('grocery', 'rotating', 'burial', 'investment', 'savings');
CREATE TYPE stokvel_status AS ENUM ('active', 'paused', 'closed');
CREATE TYPE member_role AS ENUM ('chairperson', 'secretary', 'treasurer', 'member');
CREATE TYPE member_status AS ENUM ('active', 'pending', 'suspended', 'exited');

CREATE TABLE stokvels (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name                VARCHAR(255) NOT NULL,
    type                stokvel_type NOT NULL,
    status              stokvel_status NOT NULL DEFAULT 'active',
    description         TEXT,
    avatar_url          TEXT,
    contribution_amount NUMERIC(12,2) NOT NULL,
    contribution_day    INTEGER NOT NULL,  -- day of month
    max_members         INTEGER,
    payout_schedule     JSONB,  -- flexible payout config
    rules               TEXT,
    constitution_signed_at TIMESTAMPTZ,
    created_by          UUID NOT NULL REFERENCES users(id),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE stokvel_members (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    stokvel_id  UUID NOT NULL REFERENCES stokvels(id) ON DELETE CASCADE,
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role        member_role NOT NULL DEFAULT 'member',
    status      member_status NOT NULL DEFAULT 'active',
    payout_position INTEGER,  -- for rotating stokvels
    joined_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    exited_at   TIMESTAMPTZ,
    UNIQUE(stokvel_id, user_id)
);

CREATE TABLE stokvel_wallets (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    stokvel_id  UUID NOT NULL UNIQUE REFERENCES stokvels(id) ON DELETE CASCADE,
    balance     NUMERIC(12,2) NOT NULL DEFAULT 0,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE multisig_approvals (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    stokvel_id      UUID NOT NULL REFERENCES stokvels(id) ON DELETE CASCADE,
    action_type     VARCHAR(50) NOT NULL,  -- withdrawal | bulk_order | rule_change
    action_payload  JSONB NOT NULL,
    requested_by    UUID NOT NULL REFERENCES users(id),
    approvals       JSONB NOT NULL DEFAULT '[]',  -- array of {user_id, approved_at}
    required_count  INTEGER NOT NULL DEFAULT 2,
    status          VARCHAR(20) NOT NULL DEFAULT 'pending',  -- pending | approved | rejected | executed
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at     TIMESTAMPTZ
);
