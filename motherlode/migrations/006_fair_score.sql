-- Fair Score system
CREATE TABLE fair_scores (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id             UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    score               INTEGER NOT NULL DEFAULT 380,
    payment_history     INTEGER NOT NULL DEFAULT 0,
    consistency         INTEGER NOT NULL DEFAULT 0,
    group_activity      INTEGER NOT NULL DEFAULT 0,
    member_tenure       INTEGER NOT NULL DEFAULT 0,
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE fair_score_history (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    score       INTEGER NOT NULL,
    delta       INTEGER NOT NULL,
    reason      TEXT NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
