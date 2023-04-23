CREATE TABLE IF NOT EXISTS highlights (
    user_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    phrase TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    last_used_at TIMESTAMPTZ,
    snooze_until TIMESTAMP,
    ignored_channels BIGINT ARRAY,
    ignored_users BIGINT ARRAY,
    PRIMARY KEY (user_id, guild_id, phrase)
)
