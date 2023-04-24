CREATE TABLE IF NOT EXISTS counters (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    author BIGINT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    last_active TIMESTAMPTZ NOT NULL,
    guild_ids BIGINT[] NOT NULL
);

CREATE TABLE IF NOT EXISTS counters_entries (
    id UUID PRIMARY KEY,
    source_counter_id UUID REFERENCES counters(id) ON DELETE CASCADE NOT NULL,
    source_guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
)
