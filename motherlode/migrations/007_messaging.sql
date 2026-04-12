-- Group chat & notifications
CREATE TYPE message_type AS ENUM ('text', 'voice_note', 'system', 'order_vote', 'payout_notice');

CREATE TABLE messages (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    stokvel_id  UUID NOT NULL REFERENCES stokvels(id) ON DELETE CASCADE,
    sender_id   UUID REFERENCES users(id),
    type        message_type NOT NULL DEFAULT 'text',
    content     TEXT,
    media_url   TEXT,
    metadata    JSONB,  -- for order_vote type: {order_id}
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE notifications (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title       VARCHAR(255) NOT NULL,
    body        TEXT NOT NULL,
    type        VARCHAR(50) NOT NULL,
    is_read     BOOLEAN NOT NULL DEFAULT false,
    metadata    JSONB,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
