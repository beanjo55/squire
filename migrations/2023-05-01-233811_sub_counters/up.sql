CREATE TABLE IF NOT EXISTS sub_counters (
    id UUID PRIMARY KEY,
    root_counter_id UUID REFERENCES counters(id) ON DELETE CASCADE NOT NULL,
    name TEXT NOT NULL,
    description TEXT
);

ALTER TABLE counters_entries ADD COLUMN sub_counter_id UUID REFERENCES sub_counters(id) ON DELETE CASCADE;