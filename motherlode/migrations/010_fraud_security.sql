-- Fraud Shield & security
CREATE TABLE fraud_settings (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id                 UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    behavioral_analytics    BOOLEAN NOT NULL DEFAULT false,
    sim_swap_detection      BOOLEAN NOT NULL DEFAULT false,
    jailbreak_detection     BOOLEAN NOT NULL DEFAULT false,
    geofencing              BOOLEAN NOT NULL DEFAULT false,
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE fraud_alerts (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id     UUID NOT NULL REFERENCES users(id),
    type        VARCHAR(50) NOT NULL,  -- sim_swap | geo_anomaly | behavior | jailbreak
    severity    VARCHAR(20) NOT NULL,  -- low | medium | high | critical
    detail      JSONB NOT NULL,
    resolved    BOOLEAN NOT NULL DEFAULT false,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE feature_lock_settings (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id             UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    lock_withdrawals    BOOLEAN NOT NULL DEFAULT false,
    lock_payments       BOOLEAN NOT NULL DEFAULT false,
    lock_statements     BOOLEAN NOT NULL DEFAULT false,
    lock_linked_accounts BOOLEAN NOT NULL DEFAULT false,
    lock_investments    BOOLEAN NOT NULL DEFAULT false,
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_sessions (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    device_id       TEXT NOT NULL,
    ip_address      INET,
    user_agent      TEXT,
    last_seen_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
