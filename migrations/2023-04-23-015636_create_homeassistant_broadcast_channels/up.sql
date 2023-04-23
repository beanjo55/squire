CREATE TABLE IF NOT EXISTS ha_broadcast_channels (
    id UUID PRIMARY KEY,
    webhook_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    last_active TIMESTAMPTZ NOT NULL
);

CREATE INDEX ha_broadcast_channels_name_idx on ha_broadcast_channels (name);