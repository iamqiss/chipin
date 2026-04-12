-- Discover / public stokvel listings
CREATE TABLE stokvel_listings (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    stokvel_id      UUID NOT NULL UNIQUE REFERENCES stokvels(id) ON DELETE CASCADE,
    is_public       BOOLEAN NOT NULL DEFAULT false,
    location        VARCHAR(255),
    latitude        NUMERIC(10,7),
    longitude       NUMERIC(10,7),
    tags            TEXT[],
    avg_return_pct  NUMERIC(5,2),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE join_requests (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    stokvel_id  UUID NOT NULL REFERENCES stokvels(id) ON DELETE CASCADE,
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    message     TEXT,
    status      VARCHAR(20) NOT NULL DEFAULT 'pending',  -- pending | approved | rejected
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(stokvel_id, user_id)
);
