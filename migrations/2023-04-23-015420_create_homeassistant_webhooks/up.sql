CREATE TABLE IF NOT EXISTS ha_webhooks (
    id UUID PRIMARY KEY,
    url TEXT NOT NULL,
    secret TEXT,
    name TEXT NOT NULL,
    description TEXT,
)

ALTER TABLE ha_webhooks ADD INDEX ha_webhooks_name_idx (name);