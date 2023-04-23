CREATE TABLE IF NOT EXISTS ha_webhooks (
    id UUID PRIMARY KEY,
    url TEXT NOT NULL,
    secret TEXT,
    name TEXT NOT NULL,
    description TEXT
);

CREATE INDEX ha_webhooks_name_idx on ha_webhooks (name);