CREATE TABLE IF NOT EXISTS counters (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    author BIGINT NOT NULL,
    description TEXT,
    created_at TIMESTAMPZ NOT NULL,
    last_active TIMESTAMPZ NOT NULL,
    guild_ids BIGINT ARRAY
)

CREATE TABLE IF NOT EXISTS counters_entries (
    id UUID PRIMARY KEY,
    source_counter_id TEXT NOT NULL,
    source_guild_id BIGINT NOT NULL
    user_id BIGINT NOT NULL,
    created_at TIMESTAMPZ NOT NULL
)
