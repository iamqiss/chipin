-- Market & bulk orders
CREATE TYPE order_status AS ENUM ('pending_votes', 'approved', 'processing', 'fulfilled', 'cancelled');

CREATE TABLE retailers (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        VARCHAR(100) NOT NULL,
    logo_url    TEXT,
    is_active   BOOLEAN NOT NULL DEFAULT true,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE market_products (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    retailer_id     UUID NOT NULL REFERENCES retailers(id),
    name            VARCHAR(255) NOT NULL,
    description     TEXT,
    image_url       TEXT,
    price           NUMERIC(12,2) NOT NULL,
    unit            VARCHAR(50) NOT NULL,
    min_quantity    INTEGER NOT NULL DEFAULT 1,
    discount_pct    NUMERIC(5,2) NOT NULL DEFAULT 0,
    category        VARCHAR(100),
    is_available    BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE group_orders (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    stokvel_id      UUID NOT NULL REFERENCES stokvels(id) ON DELETE CASCADE,
    retailer_id     UUID NOT NULL REFERENCES retailers(id),
    items           JSONB NOT NULL,  -- [{product_id, quantity, unit_price}]
    total_amount    NUMERIC(12,2) NOT NULL,
    status          order_status NOT NULL DEFAULT 'pending_votes',
    delivery_option VARCHAR(20) NOT NULL DEFAULT 'collection',  -- collection | delivery
    created_by      UUID NOT NULL REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE order_votes (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id    UUID NOT NULL REFERENCES group_orders(id) ON DELETE CASCADE,
    user_id     UUID NOT NULL REFERENCES users(id),
    vote        BOOLEAN NOT NULL,
    voted_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(order_id, user_id)
);

CREATE TABLE collection_codes (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id    UUID NOT NULL REFERENCES group_orders(id),
    user_id     UUID NOT NULL REFERENCES users(id),
    code        VARCHAR(20) NOT NULL UNIQUE,
    is_used     BOOLEAN NOT NULL DEFAULT false,
    used_at     TIMESTAMPTZ,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
