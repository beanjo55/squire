CREATE TABLE IF NOT EXISTS ha_broadcast_channels (
    id UUID PRIMARY KEY,
    webhook_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPZ NOT NULL,
    last_active TIMESTAMPZ NOT NULL,
)

ALTER TABLE ha_broadcast_channels ADD INDEX ha_broadcast_channels_name_idx (name);